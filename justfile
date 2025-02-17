# 定义变量
TARGET := "nezamq"
BUILD_FOLD := "./build"
VERSION := `cat version.ini`
PACKAGE_FOLD_NAME := TARGET + "-" + VERSION
TARBALL := PACKAGE_FOLD_NAME + ".tar.gz"

release:
    mkdir -p {{BUILD_FOLD}}
    cargo build --release

    mkdir -p {{BUILD_FOLD}}/{{PACKAGE_FOLD_NAME}}
    mkdir -p {{BUILD_FOLD}}/{{PACKAGE_FOLD_NAME}}/bin
    mkdir -p {{BUILD_FOLD}}/{{PACKAGE_FOLD_NAME}}/libs
    mkdir -p {{BUILD_FOLD}}/{{PACKAGE_FOLD_NAME}}/config

    cp -rf target/release/placement-center {{BUILD_FOLD}}/{{PACKAGE_FOLD_NAME}}/libs
    cp -rf bin/* {{BUILD_FOLD}}/{{PACKAGE_FOLD_NAME}}/bin
    cp -rf config/* {{BUILD_FOLD}}/{{PACKAGE_FOLD_NAME}}/config
    chmod -R 777 {{BUILD_FOLD}}/{{PACKAGE_FOLD_NAME}}/bin/*

    cd {{BUILD_FOLD}} && tar zcvf {{TARBALL}} {{PACKAGE_FOLD_NAME}} && rm -rf {{PACKAGE_FOLD_NAME}}
    echo "build release package success. {{TARBALL}}"

test:
    sh ./scripts/integration-testing.sh

clean:
    cargo clean
    rm -rf build