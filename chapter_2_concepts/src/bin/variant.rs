/**
 * 变量和常量
 */
fn main() {
    // 变量默认是不可变的, 如果需要改变变量的值, 需要在变量名前加mut关键字
    // let x = 5;
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //常量, 常量不允许使用mut关键字(不允许可变), 并且必须指定类型
    // 常量的命名规范是使用全大写字母, 并使用下划线分隔单词
    // 常量需要在声明时就赋值, 并且常量的值不允许改变, 和不可变变量不同, 常量的值在编译时就确定了, 变量的值可以在运行时赋值
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // 变量遮蔽(shadowing), 可以在同一个作用域内声明一个同名变量, 但是新变量会覆盖旧变量, 旧变量依然存在, 但是在当前作用域内不可见
    let x = 5;
    let x = x + 1;
    // 变量遮蔽可以改变变量的类型, 可变变量不允许改变变量的类型
    {
        let x = "hello";
        println!("The value of x in inner scope is: {x}");
    }
    
    let x = x * 2;
    println!("The value of x is: {x}");
}
