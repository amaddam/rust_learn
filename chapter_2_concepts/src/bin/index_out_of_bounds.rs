use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);



    loop {
        let mut index = String::new();
        println!("please input an index.");

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line.");

        let index: usize = match { index.trim().parse() } {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                continue;
            },
        };

        match a.get(index) {
            Some(&value) => {
                println!("The value of a[{index}] is: {value}");
                break;
            },
            None => println!("The index is out of bounds."),
        }
    }
}
