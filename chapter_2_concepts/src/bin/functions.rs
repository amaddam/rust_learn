fn main() {
    println!("Hello, world!");
    another_function(11);
    print_labeled_measurement(10, 'h');
    //赋值语句并不返回值, 所以会报错
    // let x = (let y = 6);

    //表达式语句, 会返回值
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    println!(
        "The value of plus_one(5) is: {result}",
        result = plus_one(5)
    );
}

// 函数定义, 参数类型必须指定
fn another_function(x: i8) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(length: i32, unit_label: char) {
    //println 函数的参数可以使用命名参数
    println!(
        "The length is: {length} {unit_label}",
        length = length,
        unit_label = unit_label
    );
    //println 函数的参数可以使用位置参数(比较旧, 但copilot会推荐使用)
    println!("The length is: {} {}", length, unit_label);
    //println 函数的参数可以使用位置参数
    println!("The length is: {0} {1}", length, unit_label);
    //println 函数的参数可以使用位置参数(位置参数可以重复使用)
    println!("The length is: {1} {0}", unit_label, length);
    //println 函数的参数可以使用命名参数, (新的语言特性, 1.26 版本引入, 推荐使用)
    println!("The length is: {length} {unit_label}");
    //println 函数的参数可以在println中使用临时变量
    println!("The length is: {length} {unit_label}", unit_label = 'k');
}

//函数的返回值类型必须指定, 格式为 fn 函数名(参数列表) -> 返回值类型
fn plus_one(x: i32) -> i32 {
    //一些语言中, 可能会使用return关键字来返回值, 但在Rust中, 不需要使用return关键字, 最后一个表达式的值会被返回
    // return x + 1;

    //这里的x + 1;是一个语句, 不是一个表达式, 所以会报错
    // x + 1;

    //这里的x + 1是一个表达式, 所以会返回值
    x + 1
}
