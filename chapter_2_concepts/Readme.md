## 说明
rust的运行模式主要有两种，debug模式(cargo run) 和release 模式 (cargo run --release) 不同模式会导致程序运行出不同的结果, 比如溢出, debug模式溢出会有报错, 而release会补码