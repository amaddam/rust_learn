# rust语言学习
[官方中文文档](https://kaisery.github.io/trpl-zh-cn)

## 前言
在学习这些之前可以先学习下cargo中包的相关概念, 代码文件在结构化之后会显示的更加容易明白先后顺序
当前为了方便学习, 我已经将仓库中的代码使用不同的文件夹进行了划分, 可以在每个章节的目录下的bin文件夹中找到对应的学习代码, 在章节目录下的demo文件夹中存在一些小的演示项目, bin中的代码可以直接使用vscode运行代码, 也可以使用cargo运行代码
vscode运行最好安装下rust-analyzer和Rust-Syntax, 这样可以更好的查看代码的结构
cargo的运行因为当前使用了包的原因, 所以会比较复杂一些, 在根目录下执行cargo run --package {package_name} --bin {bin_name}即可运行bin目录下的代码, 如果是bin目录下的文件, 那么{bin_name}就是文件名, 如果不是, 那需要在Cargo.toml中查找

比如需要运行chapter_3_concepts目录下的chapter_3_1_variant.rs文件, 那么只需要在根目录下执行cargo run --package chapter_3_concepts --bin chapter_3_1_variant即可运行
```bash
cargo run --package chapter_3_concepts --bin chapter_3_1_variant
```

而如果需要运行chapter_3_concepts目录下demo目录下的fibonacci.rs文件, 那么需要在根目录下执行cargo run --package chapter_3_concepts --bin demo_02_fibonacci才可运行
可以看到demo_02_fibonacci和fibonacci.rs并不是对应的, 这是因为在chapter_3_concepts目录下的demo目录下的Cargo.toml文件中的[[bin]]定义了这个名字
参考如下:
Cargo.toml
```toml
[[bin]]
name = "demo_02_fibonacci"
path = "src/bin/demo/fibonacci.rs"
```
执行命令
cargo run --package {package_name} --bin {bin_name(这里的名字是Cargo.toml中定义的name)}
```bash
cargo run --package chapter_3_concepts --bin demo_02_fibonacci
```


## 命令

1. 常用Rust相关命令
    1. rustup update 更新rust版本
    2. rustup show 显示当前的rust版本
    3. rustc --version 显示当前的rust版本
    4. rustc {file_name}.rs 编译一个文件
    5. ./{file_name} 运行一个文件

2. cargo相关命令
    1. 创建一个新的二进制包

        `
        cargo new {project_name} --bin 
        `

        会在当前目录下创建一个新的包, 文件夹中包含了一个Cargo.toml文件和src/main.rs文件, --bin(默认) 可以省略, 默认创建的是二进制包, 如果需要创建一个库, 那么需要使用--lib

    2. 