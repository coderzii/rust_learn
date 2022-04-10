use rand::Rng; //trait
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("猜一个数");

        let mut guess_number = String::new();

        //println!("神秘数字是:{}", secret_number);

        io::stdin()
            .read_line(&mut guess_number)
            .expect("无法读取行");

        println!("你猜测的数是:{}", guess_number);

        let guess_number: u32 = match guess_number
            .trim() // \n
            .parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        //.expect("please input a number !");

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("too small !"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
