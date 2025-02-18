# nezamq
rust study project

## 任务一： 项目结构：组织、编译、打包rust项目

```
├── Cargo.toml # Cargo 的定义文件
├── LICENSE # 项目 的LICENSE 文件，比如Apache2.0 (暂无)
├── README.md # 项目说明 README文件
├── benches # 压测代码所在目录
├── bin # 项目启动命令存放的目录
├── build.rs # cargo中的构建脚本 build.rs。可参考这个文档:https://course.rs/cargo/reference/build-script/intro.html
├── config # 存放配置文件的目录
├── docs # 存放技术文档的目录
├── example # 存放项目调用 Demo 的项目
├── Makefile # 用于编译打包项目的makefile文件
├── Justfile # 用于编译打包项目的justfile文件
├── src  # 源代码目录
│   ├── cmd
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   └── placement-center
│   │   │       └── server.rs
│   │   └── tests
│   │       └── test.rs
│   ├── placement-center
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   └── lib.rs
│   │   └── tests
│   │       └── test.rs
│   └── protocol
│       ├── Cargo.toml
│       ├── src
│       │   └── lib.rs
│       └── tests
│           └── test.rs
├── tests # 存放测试用例代码的文件
└── version.ini  # 记录版本信息的文件
```

## 任务二：项目初始化：命令行参数、配置、日志、测试用例

### 1. 命令行参数

Rust处理命令行参数推荐使用 clap 库。

```
cargo add clap -F derive

```
需求目标：能从命令行接受配置文件路径
静态文件配置：使用toml文件，使用toml库
