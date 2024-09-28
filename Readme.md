# rust语言学习
[官方文档](https://kaisery.github.io/trpl-zh-cn)

在学习这些之前需要有些学习下命名空间, 代码文件在结构化之后会显示的更加容易明白先后顺序
当前为了方便学习, 我已经提前将仓库中的代码使用命名空间进行了划分, 你可以在每个章节的目录下找到对应的代码, 可以直接使用vscode运行代码, 也可以使用cargo运行代码
vscode运行最好安装下rust-analyzer和Rust-Syntax, 这样可以更好的查看代码的结构
cargo的运行因为当前使用了命名空间的原因, 所以会比较复杂一些, 在根目录下执行cargo run --package {package_name} --bin {bin_name}即可运行bin目录下的代码

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