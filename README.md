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

### 2.配置
需求目标：能从命令行接受配置文件路径
静态文件配置：使用toml文件，使用toml库

### 3. 日志

日志核心四个需求：

1. 支持多个不同的日志级别。
2. 支持多种日志滚动方式，比如按时间滚动、按大小滚动。
3. 支持自定义日志格式。
4. 支持根据不同的类型将日志打印到不同的文件。

日志库： log4rs

初始化日志模块主要分为三步：

1. 编写 log4rs.yaml 文件。
2. 初始化日志模块。
3. 记录日志。

用户使用日志，一般只需要去修改存储路径，这个是常见需求
希望用户不部份情况下不用起理解 log4rs 的语法，且修改日志存放目录时，不需要修改 log4rs.yaml 中的路径

### 4. 运行测试用例
一般会把测试用例和代码写在一起。

测试用例的逻辑：初始化某个数据，然后判断数据是否符合预期。

测试Server来提供服务，使用脚本来测试。

```shell
#!/bin/sh
start_placement_center(){
    nohup cargo run --package cmd --bin $placement_center_process_name -- --conf=tests/config/$placement_center_process_name.toml >/dev/null 2>&1 &
    sleep 3
    while ! ps aux | grep -v grep | grep "$placement_center_process_name" > /dev/null; do
        echo "Process $placement_center_process_name has not started yet, wait 1s. . . ."
        sleep 1
    done
    echo "Process $placement_center_process_name starts successfully and starts running the test case"
}

stop_placement_center(){
    pc_no= `ps aux | grep -v grep | grep "$placement_center_process_name" | awk '{print $2}'
    echo "placement center num: &pc_no"
    kill $pc_no
    sleep 3

    while ps aux | grep -v grep | grep "$placement_center_process_name" > /dev/null; do
        echo "Process $placement_center_process_name stopped successfully "
        sleep 1
    done
}

start_placement_center

cargo test

if [ $? -ne 0 ]; then
    echo "Test case failed to run"
    stop_placement_center
    exit 1
else
    echo "Test case runs successfully"
    stop_placement_center
fi

```
shell 脚本说明：

```
nohup: 用于让命令在后台运行，即使终端关闭，进程也不会被终止。

cargo run: 运行 Rust 项目。

--package cmd: 指定要运行的 Cargo 工作空间中的包（cmd 是包名）。

--bin $placement_center_process_name: 指定要运行的二进制文件名称（$placement_center_process_name 是一个变量，表示二进制文件名）。

-- --conf=tests/config/$placement_center_process_name.toml: 传递给 Rust 程序的参数，指定配置文件路径。

>/dev/null 2>&1: 将标准输出（stdout）和标准错误（stderr）重定向到 /dev/null，即丢弃所有输出。 
(将 nohup 的输出重定向到日志文件，而不是丢弃,可以写成 nohup cargo run --package cmd --bin $placement_center_process_name -- --conf=tests/config/$placement_center_process_name.toml > app.log 2>&1 &)

&: 将命令放到后台运行。

ps aux: 列出当前系统中所有进程的信息。

grep -v grep: 过滤掉 grep 命令本身的进程（因为 grep 也会出现在进程列表中）。

grep "$placement_center_process_name": 查找包含 $placement_center_process_name 的进程。

> /dev/null: 将 grep 的输出重定向到 /dev/null，即丢弃输出（只关心命令的退出状态）。

while ! ...: 如果 grep 没有找到匹配的进程（即进程未启动），则进入循环。

echo "Process $placement_center_process_name has not started yet, wait 1s....": 输出提示信息，表示进程尚未启动。

sleep 1: 等待 1 秒后再次检查。

ps aux: 列出当前系统中所有进程的详细信息。

grep -v grep: 过滤掉 grep 命令本身的进程（因为 grep 也会出现在进程列表中）。

grep "$placement_center_process_name": 查找包含 $placement_center_process_name 的进程。

awk '{print $2}': 提取进程列表的第二列，即进程 ID（PID）。

pc_no=: 将提取到的进程 ID 赋值给变量 pc_no。

kill $pc_no: 向进程 ID 为 $pc_no 的进程发送终止信号（默认是 SIGTERM），要求进程优雅退出。
（强制终止: 如果进程没有响应 SIGTERM，可以使用 kill -9 $pc_no 强制终止进程。）

ps aux | grep -v grep | grep "$placement_center_process_name": 检查是否还存在名为 $placement_center_process_name 的进程。

> /dev/null: 将 grep 的输出重定向到 /dev/null，即丢弃输出（只关心命令的退出状态）。

while ...: 如果进程仍然存在（即未成功停止），则进入循环。

echo "”Process $placement_center_process_name stopped successfully": 输出提示信息，表示进程仍在运行。

sleep 1: 等待 1 秒后再次检查。

多进程处理: 如果存在多个同名进程，当前脚本只会终止第一个找到的进程。可以使用循环终止所有同名进程：
for pid in $(ps aux | grep -v grep | grep "$placement_center_process_name" | awk '{print $2}'); do
    kill $pid
done

$?: 这是一个特殊变量，表示上一个命令的退出状态码。
    如果命令成功执行，$? 的值为 0。
    如果命令执行失败，$? 的值非 0。

-ne: 表示“不等于”（not equal）。

[ $? -ne 0 ]: 判断上一个命令的退出状态码是否不等于 0（即是否执行失败）。
```
