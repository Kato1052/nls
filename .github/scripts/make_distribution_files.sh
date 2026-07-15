#! /bin/sh

TAG=$1
PRODUCT_NAME=nls
RELEASE=$PRODUCT_NAME-$TAG-arm64-darwin

cargo build --release
mkdir -p dist/$RELEASE
cp LICENSE README.md dist/$RELEASE

tar cvfz dist/$RELEASE.tar.gz -C dist $RELEASE
