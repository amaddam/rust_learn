## 学习补充说明
rust的运行模式主要有两种: debug模式 (`cargo run`) 和release模式 (`cargo run --release`). 不同模式会导致程序运行出不同的结果, 比如溢出. debug模式溢出会报错, 而release模式会采用补码处理.
