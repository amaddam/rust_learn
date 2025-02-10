fn main() {
    //字符串字面值是不可变的, 使不使用mut修饰都可以, 使用会有警告
    let /*mut*/ s = "Hello World!";
    //字面值并没有提供修改的方法
    println!("&s: {}", s);

    //String 是可变的, 使用mut修饰
    let mut s = String::from("hello");
    println!("{s}");
    //push_str() 方法将字符串附加到 String
    s.push_str(", world!");
    println!("{s}");

    let mut n = String::new();
    println!("Please enter a number.");
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let mut x: i8 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a legal number");
            0
        }
    };

    //变量在复制后, 之前的变量就不能再使用了
    let m = n;
    println!("m: {m}");
    // println!("n: {n}, m: {m}");


    //如果是整数, 这种编译时就能确定大小的类型, 是可以复制的, 或者一个类型实现了Copy trait, 也是可以复制的
    let y = x;
    let z = x.clone();
    println!("x: {x}, y: {y}");
    //且原来的变量还可以继续使用
    x += 1;
    println!("x: {x}, z: {z}");

    // s 进入作用域
    let s = String::from("hello");

    // s 的值移动到函数里 ...
    takes_ownership(s);
    // ... 所以到这里s不再有效
    // println!("{s}");

    // x 进入作用域
    let x = 5; 
    println!("integer pretreatment: {x}");
    makes_copy(x); 
    // x 应该到函数里，
    // 但 i32 是 Copy 的，
    //所以在后面可继续使用 x
    println!("integer after function: {x}");
} 
// 这里, x 先离开作用域，然后是 s。但因为 s 的值已被移走，
// 所以不会有特殊操作

fn takes_ownership(mut some_string: String) {
    // some_string 进入作用域
    some_string.push_str(", world");
    println!("not copy {some_string}");
} // 这里, some_string 离开作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(mut some_integer: i32) {
    // some_integer 进入作用域
    some_integer += 1;
    println!("integer in function: {some_integer}");
} // 这里, some_integer 离开作用域。由于它是 Copy 的，所以不会有特殊操作
