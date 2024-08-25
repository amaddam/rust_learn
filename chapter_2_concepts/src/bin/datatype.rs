use std::{i8, u16};

fn main() {
    // rust 是静态类型语言, 编译期必须知道所有变量的类型, 如果多种类型均有可能, 需要显式指定类型
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");

    // 标量(Scalar)类型, 代表单个值, 4种基本类型: 整型, 浮点型, 布尔型, 字符型
    // 整型, 长度, 8位, 16位, 32位(默认), 64位, 128位, isize和usize(有符号和无符号, 与操作系统位数相, 如果是64位操作系统, 则为64位, 32位操作系统则为32位)
    let i8: i8 = -128;
    println!("The value of i8 is: {i8}");
    let u16: u16 = 65535;
    println!("The value of u16 is: {u16}");
    let signed: isize = -1;
    println!("The value of signed is: {signed}");
    let unsigned: usize = 1;
    println!("The value of unsigned is: {unsigned}");

    //整形溢出, debug模式下会panic, release模式下会进行二进制补码运算, 值256会变成0, 257会变成1
    //release模式需要使用--release参数编译, cargo run -p chapter_2_concepts --release --bin datatype
    // let mut overflow: u8 = 255;
    //因为debug模式下会panic, 所以需要注释掉这行代码
    // overflow = overflow + 1;
    // println!("The value of overflow is: {overflow}");

    //4种处理溢出的方法
    //1. wrapping_add, wrapping_sub, wrapping_mul, wrapping_div, wrapping_rem, wrapping_shl, wrapping_shr, wrapping_neg, wrapping_abs
    //wrapping_add, 会进行二进制补码运算, 255 + 1 = 0, 256 + 1 = 1
    let mut overflow: u8 = 255;
    overflow = overflow.wrapping_add(1);
    println!("The value of overflow produced by wrapping_add is: {overflow}");

    //2. checked_add, checked_sub, checked_mul, checked_div, checked_rem, checked_shl, checked_shr, checked_neg, checked_abs
    //checked_add, 会返回Option, 如果溢出返回None, 否则返回Some
    let overflow: u8 = 255;
    match overflow.checked_add(1) {
        Some(v) => println!("The value of overflow produced by checked_add is: {v}"),
        None => println!("The value of overflow produced by checked_add is: overflow"),
    }

    //3. saturating_add, saturating_sub, saturating_mul, saturating_div, saturating_rem, saturating_shl, saturating_shr
    //saturating_add, 如果溢出返回u8的最大值255, 255 + 1 = 255
    let overflow: u8 = 255;
    let v = overflow.saturating_add(1);
    println!("The value of overflow produced by saturating_add is: {v}");

    //4. overflowing_add, overflowing_sub, overflowing_mul, overflowing_div, overflowing_rem, overflowing_shl, overflowing_shr
    //overflowing_add, 会返回元组, 第一个元素是计算结果, 第二个元素是是否溢出, 255 + 1 = (0, true)
    let overflow: u8 = 255;
    let (v, overflowed) = overflow.overflowing_add(1);
    println!("The value of overflow produced by overflowing_add is: {v}, overflowed: {overflowed}");

    //浮点型, 2种, f32, f64, 默认是f64, 浮点型都是有符号的
    let float: f32 = 3.0;
    println!("The value of float 32 is: {float}");
    let float: f64 = 3.0;
    println!("The value of float 64 is: {float}");

    //布尔型, 1字节, true, false
    let boolean: bool = true;
    println!("The value of boolean is: {boolean}");

    //字符型, 4字节, 单引号, Unicode标量值, 可以使用\u{xxxx}表示Unicode标量值
    let character: char = '😻';
    println!("The value of character is: {character}");

    //复合类型, 代表多个值, 2种, 元组, 数组
    //元组, 固定长度, 一旦声明, 长度不可变, 可以包含不同类型的值
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    //使用{:?}打印元组,会展示在一行里, 而使用{:#?}打印元组, 可以更好的展示元组的结构, 每个元素占一行
    println!("The value of tuple is: {:#?}", tuple);
    //可以使用模式匹配解构元组
    let (x, y, z) = tuple;
    println!("The value of x is: {x}, y is: {y}, z is: {z}");
    //可以使用.操作符访问元组的元素
    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;
    println!("The value of x is: {x}, y is: {y}, z is: {z}");

    //数组, 固定长度, 一旦声明, 长度不可变, 必须包含相同类型的值
    //可以这样声明数组, [3, 3, 3, 3, 3], [初始化值; 元素个数], [3; 5]
    let array = [3; 5];
    println!("The value of array is: {:#?}", array);
    //也可以这样声明数组, [1, 2, 3, 4, 5], [元素1, 元素2, 元素3, 元素4, 元素5], 类型和元素个数可以省略, 编译器会根据元素推断类型和元素个数
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of array is: {:#?}", array);

    //可以使用模式匹配解构数组
    let [a, b, c, d, e] = array;
    println!("The value of a is: {a}, b is: {b}, c is: {c}, d is: {d}, e is: {e}");
    //可以使用索引访问数组的元素
    let a = array[0];
    let b = array[1];
    let c = array[2];
    let d = array[3];
    let e = array[4];
    println!("The value of a is: {a}, b is: {b}, c is: {c}, d is: {d}, e is: {e}");
    //数组越界, 会panic
    // let array: [i32; 5] = [1, 2, 3, 4, 5];
    let element = array[5];
    println!("The value of array[5] is: {element}");
}
