use clap::{Command, CommandFactory};
use clap_complete::Shell;
use std::path::Path;

fn generate_impl(s: Shell, app: &mut Command, appname: &str, outdir: &Path, file: String) {
    let destfile = outdir.join(file);
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    if let Ok(mut dest) = std::fs::File::create(destfile) {
        clap_complete::generate(s, app, appname, &mut dest);
    }
}
pub(super) fn generate(outdir: &Path) {
    use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
    let appname = "nls";
    let mut app = crate::Args::command();
    app.set_bin_name(appname);
    generate_impl(Bash, &mut app, appname, outdir, format!("bash/{appname}"));
    generate_impl(
        Elvish,
        &mut app,
        appname,
        outdir,
        format!("elvish/{appname}"),
    );
    generate_impl(Fish, &mut app, appname, outdir, format!("fish/{appname}"));
    generate_impl(
        PowerShell,
        &mut app,
        appname,
        outdir,
        format!("powershell/{appname}"),
    );
    generate_impl(Zsh, &mut app, appname, outdir, format!("zsh/_{appname}"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_generate_creates_output_directory() {
        let dir = tempdir().unwrap();
        let outdir = dir.path().join("completions");
        
        generate(&outdir);
        
        // Check that the directory was created
        assert!(outdir.exists());
        assert!(outdir.is_dir());
    }

    #[test]
    fn test_generate_creates_bash_completion() {
        let dir = tempdir().unwrap();
        let outdir = dir.path();
        
        generate(&outdir);
        
        let bash_file = outdir.join("bash/nls");
        assert!(bash_file.exists(), "Bash completion file should be created");
        let content = fs::read_to_string(&bash_file).unwrap();
        assert!(!content.is_empty(), "Bash completion file should not be empty");
    }

    #[test]
    fn test_generate_creates_zsh_completion() {
        let dir = tempdir().unwrap();
        let outdir = dir.path();
        
        generate(&outdir);
        
        let zsh_file = outdir.join("zsh/_nls");
        assert!(zsh_file.exists(), "Zsh completion file should be created");
        let content = fs::read_to_string(&zsh_file).unwrap();
        assert!(!content.is_empty(), "Zsh completion file should not be empty");
    }

    #[test]
    fn test_generate_creates_fish_completion() {
        let dir = tempdir().unwrap();
        let outdir = dir.path();
        
        generate(&outdir);
        
        let fish_file = outdir.join("fish/nls");
        assert!(fish_file.exists(), "Fish completion file should be created");
        let content = fs::read_to_string(&fish_file).unwrap();
        assert!(!content.is_empty(), "Fish completion file should not be empty");
    }

    #[test]
    fn test_generate_creates_powershell_completion() {
        let dir = tempdir().unwrap();
        let outdir = dir.path();
        
        generate(&outdir);
        
        let ps_file = outdir.join("powershell/nls");
        assert!(ps_file.exists(), "PowerShell completion file should be created");
        let content = fs::read_to_string(&ps_file).unwrap();
        assert!(!content.is_empty(), "PowerShell completion file should not be empty");
    }

    #[test]
    fn test_generate_creates_elvish_completion() {
        let dir = tempdir().unwrap();
        let outdir = dir.path();
        
        generate(&outdir);
        
        let elvish_file = outdir.join("elvish/nls");
        assert!(elvish_file.exists(), "Elvish completion file should be created");
        let content = fs::read_to_string(&elvish_file).unwrap();
        assert!(!content.is_empty(), "Elvish completion file should not be empty");
    }

    #[test]
    fn test_generate_all_shell_types_created() {
        let dir = tempdir().unwrap();
        let outdir = dir.path();
        
        generate(&outdir);
        
        // Verify all shell directories exist
        assert!(outdir.join("bash").exists());
        assert!(outdir.join("zsh").exists());
        assert!(outdir.join("fish").exists());
        assert!(outdir.join("powershell").exists());
        assert!(outdir.join("elvish").exists());
    }

    #[test]
    fn test_generate_creates_nested_directories() {
        let dir = tempdir().unwrap();
        let outdir = dir.path();
        
        generate(&outdir);
        
        // Verify nested directory structure
        assert!(outdir.join("bash/nls").exists());
        assert!(outdir.join("zsh/_nls").exists());
        assert!(outdir.join("fish/nls").exists());
    }

    #[test]
    fn test_generate_into_existing_directory() {
        let dir = tempdir().unwrap();
        let outdir = dir.path();
        
        // Call generate twice
        generate(&outdir);
        let bash_file_first = fs::read_to_string(outdir.join("bash/nls")).unwrap();
        
        generate(&outdir);
        let bash_file_second = fs::read_to_string(outdir.join("bash/nls")).unwrap();
        
        // Files should be recreated with same content
        assert_eq!(bash_file_first, bash_file_second);
    }
}
