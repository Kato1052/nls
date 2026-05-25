# nls

ls without month abbreviation. This project was created as part of a class assignment.

[![License](https://shields.io/badge/License-MIT-blue)](https://github.com/Kato1052/nls/blob/main/LICENSE) [![build](https://github.com/Kato1052/nls/actions/workflows/build.yaml/badge.svg)](https://github.com/Kato1052/nls/actions/workflows/build.yaml) [![Coverage Status](https://coveralls.io/repos/github/Kato1052/nls/badge.svg?branch=main)](https://coveralls.io/github/Kato1052/nls?branch=main)
## Overview
`ls -l` 実行時に表示される英語の月の略称を、数字に変更したコマンド。
- `ls -l` の実行結果にはファイルの変更日時が表示されるが、月の表記が英語であるため分かりにくい。
- `nls` は `-l` を使用した場合に、変更日時を `月 日 時刻` で表示する。
    - 月は数字で表示する
    - 変更日時が実行時刻の前年度以前である場合、 `月 日 (年)` で表示する。
## Usage
```
SYNOPSIS
    nls [-l] [FILES...]

DESCRIPTION
    nls displays file names likes ls.

    The following option is available:

    nls -l
        Display extended file metadata, similar to "ls -l" command. However, it doesn't use the month abbreviation.
```
## Installation
## About
### Developer
- Kato Mizuki
### License
- MIT License
- Copyright (c) 2026 Kato1052
