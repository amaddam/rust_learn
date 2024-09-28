fn main() {
    let mut n = String::new();

    loop {
        n.clear();
        println!("Please enter a number.");
        std::io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: i8 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a legal number");
                continue;
            }
        };

  
        // 循环
        let result = fib_loop(n);
        println!("fib_loop({n}) = {result}");

        // 递归
        let result = fib_recursion(n);
        println!("fib_recursion({n}) = {result}");

        // 带有缓存的递归
        let result = fib_recursion_with_cache(n);
        println!("fib_recursion_with_cache({n}) = {result}");
    }
}

fn fib_loop(n: i8) -> i64 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 1;
    }
    let mut a = 1;
    let mut b = 1;
    let mut c = 0;
    //从2开始, 因为0和1已经计算过了, ..=n表示包含n
    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }
    c
}

fn fib_recursion(n: i8) -> i64 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 1;
    }
    fib_recursion(n - 1) + fib_recursion(n - 2)
}

fn fib_recursion_with_cache(n: i8) -> i64 {
    //todo 全局变量还没了解comp
    println!("fib_recursion_with_cache can't work now {n}");
    0
}
