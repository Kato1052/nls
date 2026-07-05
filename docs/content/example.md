---
date: '2026-07-05T12:17:33+09:00'
draft: true
title: 'Example'
---

## nlsの表示
ファイルの変更時刻が分かりやすい
```
% nls -l
-rw-r--r--  1 km staff      0  1/ 1 (1970) an_old_file
-rw-r--r--  1 km staff  21643  7/ 4  11:29 Cargo.lock
-rw-r--r--  1 km staff    213  7/ 4  11:29 Cargo.toml
-rw-r--r--  1 km staff    912  7/ 4  11:29 Dockerfile
drwxr-xr-x 15 km staff    480  7/ 4  11:29 docs/
-rw-r--r--  1 km staff   1124  7/ 4  11:29 Justfile
-rw-r--r--  1 km staff   1065  7/ 4  11:29 LICENSE
-rw-r--r--  1 km staff   1444  7/ 5  12:13 README.md
drwxr-xr-x  4 km staff    128  7/ 4  11:29 src/
drwxr-xr-x  6 km staff    192  7/ 3  21:16 target/
```

## システムが日本語の場合のls
月には月が付いているのに、日付は数字のみで美しくないし、何を表している数字なのか分かりにくい
```
% ls -l
total 88
-rw-r--r--@  1 km  staff      0  1月  1  1970 an_old_file
-rw-r--r--@  1 km  staff  21643  7月  4 11:29 Cargo.lock
-rw-r--r--@  1 km  staff    213  7月  4 11:29 Cargo.toml
-rw-r--r--@  1 km  staff    912  7月  4 11:29 Dockerfile
drwxr-xr-x@ 15 km  staff    480  7月  4 11:29 docs
-rw-r--r--@  1 km  staff   1124  7月  4 11:29 Justfile
-rw-r--r--@  1 km  staff   1065  7月  4 11:29 LICENSE
-rw-r--r--@  1 km  staff   1444  7月  5 12:13 README.md
drwxr-xr-x@  4 km  staff    128  7月  4 11:29 src
drwxr-xr-x@  6 km  staff    192  7月  3 21:16 target
```

## システムが英語の場合のls
月が英語で、何月なのか分かりにくい
```
ls -l
total 52
-rw-rw-r-- 1 km km     0 Jan  1  1970 an_old_file
-rw-rw-r-- 1 km km 21643 Jul  5 12:37 Cargo.lock
-rw-rw-r-- 1 km km   213 Jul  5 12:37 Cargo.toml
-rw-rw-r-- 1 km km   912 Jul  5 12:37 Dockerfile
drwxrwxr-x 6 km km  4096 Jul  5 12:37 docs/
-rw-rw-r-- 1 km km  1124 Jul  5 12:37 Justfile
-rw-rw-r-- 1 km km  1065 Jul  5 12:37 LICENSE
-rw-rw-r-- 1 km km  1444 Jul  5 12:37 README.md
drwxrwxr-x 2 km km  4096 Jul  5 12:37 src/
```

## lsのオプションでフォーマットを指定した場合
古いファイルの場合、日付の位置がずれて分かりにくい
```
% ls -l --time-style iso
total 52
-rw-rw-r-- 1 km km     0 1970-01-01  an_old_file
-rw-rw-r-- 1 km km 21643 07-05 12:37 Cargo.lock
-rw-rw-r-- 1 km km   213 07-05 12:37 Cargo.toml
-rw-rw-r-- 1 km km   912 07-05 12:37 Dockerfile
drwxrwxr-x 6 km km  4096 07-05 12:37 docs/
-rw-rw-r-- 1 km km  1124 07-05 12:37 Justfile
-rw-rw-r-- 1 km km  1065 07-05 12:37 LICENSE
-rw-rw-r-- 1 km km  1444 07-05 12:37 README.md
drwxrwxr-x 2 km km  4096 07-05 12:37 src/
```
