fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 5;
    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    //这里的number的类型是i32, 而不是&str, 所以会报错
    // let number = if condition { 5 } else { "six" };

    //循环
    println!("====================");
    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            //这里不太清楚为什么又要写分号了? 这里写不写都行
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    let mut count = 0;
    'counting_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        // 这里的break 'counting_loop; 会跳出到'counting_loop这个标签所在的循环
        // 这里的break; 会跳出到最近的循环
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    //while循环
    println!("====================");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    //使用while循环来遍历数组, 这种方式不太推荐, 如果长度发生变化, 会导致数组越界
    println!("使用while循环来遍历数组");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //使用for循环来遍历数组的全部元素
    println!("使用for循环来遍历数组的全部元素");
    for element in a {
        println!("the value is: {}", element);
    }

    //使用for循环来循环一个范围
    println!("使用for循环来循环一个范围");
    // 这里的..表示不包含4, 而..=表示包含4, 前面的数字表示开始, 后面的数字表示结束, 开始数字必须小于等于结束数字
    // 打印时会打印3! 2! 1!, 因为rev()方法会反转范围
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
