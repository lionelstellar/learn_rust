##  learn_rust

### 1. 安装

```
$ curl https://sh.rustup.rs -sSf | sh

// vscode插件安装
rust-analyzer   // program plugin
CodeLLDB        // debug plugin

```

### 2. 教程

https://kaisery.gitbooks.io/trpl-zh-cn/content/

### 3. 命令行编译与运行

使用rustc

```
编写hello.rs源码
$ rustc hello.rs    //编译生成二进制hello
$ ./hello           //运行二进制
```

使用cargo
```
$ cargo new $(project_name)
$ cd $(project_name)
修改Cargo.toml
编写main.rs源码
$ cargo build   //构建
$ cargo run     //构建并运行
```