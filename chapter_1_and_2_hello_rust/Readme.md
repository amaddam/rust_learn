# 说明
cargo 是 Rust 的构建系统和包管理工具。大多数情况下，你可以使用 cargo build(编译) 来构建你的项目，使用 cargo run(运行) 来运行你的项目。cargo 会自动下载你的项目的依赖，并且编译这些依赖。当然直接cargo run会更方便，因为它会自动编译并运行你的项目。
编译会将源码编译后生成一个可执行文件，这个文件会放在target/debug目录下，如果你使用了release模式，那么文件会放在target/release目录下。
```bash
cargo build
cargo run
cargo run --release
```