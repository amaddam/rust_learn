fn main() {
    // 变量默认是不可变的, 如果需要改变变量的值, 需要在变量名前加mut关键字
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
