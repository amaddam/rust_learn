fn main() {
    //摄氏度 = (华氏度 - 32) * 5 / 9
    //华氏度 = 摄氏度 * 9 / 5 + 32
    //(100度是摄氏度的水沸点, 212℉是华氏度的水沸点, 0℃是摄氏度的水冰点, 32℉是华氏度的水冰点, 摄氏度从水的冰点到沸点有100℃, 华氏度从水的冰点到沸点有180℉, 所以氏度和华氏度的比例是5:9, 同时华氏度的冰点是32℉)

    //华氏度最低为-459.67
    const MIN_CELSIUS: f64 = -459.67;
    //摄氏度最低为-273.15
    const MIN_FAHRENHEIT: f64 = -273.15;

    //1.摄氏度转换成华氏度模式
    const CELSIUS_TO_FAHRENHEIT: i8 = 1;
    //2.华氏度转换成摄氏度模式
    const FAHRENHEIT_TO_CELSIUS: i8 = 2;

    'input: loop {
        println!("================");
        //1. 输入单位 1.摄氏度 2.华氏度, 可以随时输入exit退出
        println!("Please enter the mode to be converted (1. Celsius to Fahrenheit, 2. Fahrenheit to Celsius): ");
        let mut mode = String::new();
        std::io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read mode");

        let mode: i8 = match mode.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if mode.trim() == "exit" {
                    break 'input;
                } else {
                    println!("Please enter a legal mode");
                    continue 'input;
                }
            }
        };

        //判断模式对不对
        if CELSIUS_TO_FAHRENHEIT == mode {
            println!("Current mode is 1. Celsius to Fahrenheit")
        } else if FAHRENHEIT_TO_CELSIUS == mode {
            println!("Current mode is 2. Fahrenheit to Celsius")
        } else {
            println!("Please enter a legal mode");
            continue 'input;
        }

        let mut temperature: f64;
        'input_temperature: loop {
            //2. 输入度数, 可以随时输入exit退出
            println!("\nPlease enter the temperature: ");
            let mut temperature_input = String::new();
            std::io::stdin()
                .read_line(&mut temperature_input)
                .expect("Failed to read temperature");
            temperature = match temperature_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    if temperature_input.trim() == "exit" {
                        break 'input;
                    } else {
                        println!("Please enter a legal temperature");
                        continue 'input_temperature;
                    }
                }
            };

            //根据模式判断温度最低值是否合法
            if CELSIUS_TO_FAHRENHEIT == mode {
                //为摄氏转华氏, 最多为273.15
                if MIN_CELSIUS > temperature {
                    println!("Please enter a legal temperature");
                    continue 'input_temperature;
                }
            } else if FAHRENHEIT_TO_CELSIUS == mode {
                //为华氏转摄氏, 最多为459.67
                if MIN_FAHRENHEIT > temperature {
                    println!("Please enter a legal temperature");
                    continue 'input_temperature;
                }
            } else {
                println!("Please enter a legal temperature");
                continue 'input_temperature;
            }
            break;  // 退出内层循环，继续执行后续代码
        }

        //3. 输出结果
        //华氏度 = 摄氏度 * 9 / 5 + 32
        //摄氏度 = (华氏度 - 32) * 5 / 9
        let convert_result;

        if CELSIUS_TO_FAHRENHEIT == mode {
            convert_result = round(temperature * 1.8 + 32.0, 2);
            println!("Celsius to Fahrenheit convert result is {convert_result}");
        } else {
            convert_result = round((temperature - 32.0) / 1.8, 2);
            println!("Fahrenheit to Celsius convert result is {convert_result}");
        }

        println!("\n\n\n")
    }

    println!("Program exited.");
}

//四舍五入
/**
 * 1. 保留小数点后n位
 * 2. 四舍六入五成双
 */
fn round(value: f64, n: i32) -> f64 {
    let mut value = value;
    let mut n = n;
    if n < 0 {
        n = 0;
    }

    let mut multiple = 1;

    //_表示不使用的变量
    for _ in 0..n {
        multiple *= 10;
    }

    //as f64表示转换成f64类型
    value = value * multiple as f64;
    let value_int = value as i64;
    let value_float = value - value_int as f64;

    //如果小数部分大于0.5, 则整数部分加1
    if value_float > 0.5 {
        value = (value_int + 1) as f64;

    //如果小数部分小于0.5, 则整数部分不变
    } else if value_float < 0.5 {
        value = value_int as f64;

    //如果小数部分等于0.5, 则整数部分为偶数时不变, 为奇数时加1
    } else {
        if value_int % 2 == 0 {
            value = value_int as f64;
        } else {
            value = (value_int + 1) as f64;
        }
    }

    //as f64表示转换成f64类型
    value / multiple as f64
}
