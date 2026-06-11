//! 簡易 'ls' ユーティリティ。
//!
//! - `-l` オプションで長形式表示（所有者・グループ・サイズ・最終更新時刻など）を行います。
//! - 隠しファイル（先頭が '.'）はデフォルトで除外されます。

use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::os::unix::fs::MetadataExt;

use chrono::{Local, TimeZone, Datelike, Timelike};
use users::{get_group_by_gid, get_user_by_uid};

/// ファイル名が '.' で始まるか判定します。隠しファイルであれば true を返します。
fn is_hidden(entry_name: &str) -> bool {
    entry_name.starts_with('.')
}

/// モードとファイル種別からパーミッション文字列（例: `drwxr-xr-x`）を作成します。
///
/// - `mode`: POSIX のモードビット
/// - `is_dir`: ディレクトリなら true
/// - `is_symlink`: シンボリックリンクなら true
///
/// 戻り値はパーミッション表記の文字列です。
fn file_mode_string(mode: u32, is_dir: bool, is_symlink: bool) -> String {
    let ftype = if is_symlink {
        'l'
    } else if is_dir {
        'd'
    } else {
        '-'
    };
    let mut s = String::with_capacity(10);
    s.push(ftype);
    let flags = [0o400, 0o200, 0o100, 0o040, 0o020, 0o010, 0o004, 0o002, 0o001];
    let chars = ['r', 'w', 'x'];
    for (i, &bit) in flags.iter().enumerate() {
        if mode & bit != 0 {
            s.push(chars[i % 3]);
        } else {
            s.push('-');
        }
    }
    s
}

/// パスのメタ情報を取得して、長形式表示用の文字列を生成します。
///
/// 表示にはファイル種別・モード、リンク数、所有者・グループ名、サイズ、更新時刻を含めます。
/// - `path`: 対象のパス
/// - `name`: 表示用のファイル名
///
/// エラー時はエラーメッセージ文字列を返します。
fn long_info(path: &Path, name: &str) -> String {
    match fs::symlink_metadata(path) {
        Ok(meta) => {
            let mode = meta.mode();
            let is_dir = meta.file_type().is_dir();
            let is_symlink = meta.file_type().is_symlink();
            let mode_str = file_mode_string(mode, is_dir, is_symlink);
            let nlink = meta.nlink();
            let uid = meta.uid();
            let gid = meta.gid();
            let owner = get_user_by_uid(uid).map(|u| u.name().to_string_lossy().into_owned()).unwrap_or_else(|| uid.to_string());
            let group = get_group_by_gid(gid).map(|g| g.name().to_string_lossy().into_owned()).unwrap_or_else(|| gid.to_string());
            let size = meta.size();
            let mtime = meta.mtime();
            let dt = Local.timestamp_opt(mtime, 0).unwrap();
            let now = Local::now();
            let time_str = if dt.year() < now.year() {
                format!("{} {} ({})", dt.month(), dt.day(), dt.year())
            } else {
                format!("{} {} {:02}:{:02}", dt.month(), dt.day(), dt.hour(), dt.minute())
            };
            format!("{} {:>2} {} {} {:>6} {} {}", mode_str, nlink, owner, group, size, time_str, name)
        }
        Err(e) => format!("nls: cannot access '{}': {}", name, e),
    }
}

/// ディレクトリ内のファイル名を収集し、隠しファイルを除外してソートして返します。
///
/// 成功時はファイル名の Vec を、失敗時は io::Error を返します。
fn list_names(path: &Path) -> io::Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            if !is_hidden(name) {
                names.push(name.to_string());
            }
        }
    }
    names.sort();
    Ok(names)
}

/// パスを表示します。ディレクトリなら一覧を、ファイルならそのエントリを表示します。
///
/// `long` が true の場合は長形式で出力します。
fn print_listing(path_str: &str, long: bool) {
    let path = Path::new(path_str);
    if path.is_dir() {
        match list_names(path) {
            Ok(names) => {
                for name in names {
                    let full = path.join(&name);
                    if long {
                        println!("{}", long_info(&full, &name));
                    } else {
                        println!("{}", name);
                    }
                }
            }
            Err(e) => eprintln!("nls: cannot access '{}': {}", path_str, e),
        }
    } else {
        // file or doesn't exist
        let base = Path::new(path_str).file_name().and_then(|s| s.to_str()).unwrap_or(path_str);
        if long {
            println!("{}", long_info(path, base));
        } else {
            println!("{}", base);
        }
    }
}

/// コマンドライン引数を解析します。
///
/// `-l` を検出すると `long = true` に設定し、その他の引数は表示対象のパスとして返します。
fn parse_args() -> (bool, Vec<String>) {
    let mut long = false;
    let mut paths = Vec::new();
    for arg in env::args().skip(1) {
        if arg == "-l" {
            long = true;
        } else {
            paths.push(arg);
        }
    }
    (long, paths)
}

/// エントリポイントです。引数が無ければカレントディレクトリを表示します。
fn main() {
    let (long, mut paths) = parse_args();
    if paths.is_empty() {
        paths.push(".".to_string());
    }
    for p in paths {
        print_listing(&p, long);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;

    #[test]
    fn test_is_hidden() {
        assert!(is_hidden(".gitignore"));
        assert!(!is_hidden("README.md"));
    }

    #[test]
    fn test_file_mode_string_regular_dir_symlink() {
        // regular file, mode 0o755
        let mode = 0o755;
        assert_eq!(file_mode_string(mode, false, false), "-rwxr-xr-x");
        // directory
        assert_eq!(file_mode_string(mode, true, false), "drwxr-xr-x");
        // symlink (symlink takes precedence)
        assert_eq!(file_mode_string(mode, true, true), "lrwxr-xr-x");
    }

    #[test]
    fn test_list_names_excludes_hidden_and_sorts() {
        let dir = tempdir().unwrap();
        let p = dir.path();
        File::create(p.join("b.txt")).unwrap();
        File::create(p.join(".secret")).unwrap();
        File::create(p.join("a.txt")).unwrap();

        let names = list_names(p).unwrap();
        // list_names sorts the results
        assert_eq!(names, vec!["a.txt".to_string(), "b.txt".to_string()]);
    }
}
