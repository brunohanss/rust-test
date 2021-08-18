use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);
    loop {
        println!("Try to guess the number please");
        let mut guess_from_user = String::new();
        io::stdin().read_line(&mut guess_from_user).expect("Failed");
        let guess_as_integer: i32 = guess_from_user
            .trim()
            .parse()
            .expect("Please type a number");

        println!("You guessed : {}", guess_from_user);

        match guess_as_integer.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Win yeahea !");
                break;
            }

            Ordering::Greater => println!("Too high"),
        }
        // if secret_number == guess_as_integer {
        //     println!("Well done!");
        // } else {
        //     println!("Not the same !")
        // }
        // println!("{}", guess_from_user.to_string());
        // println!("{}", secret_number);
    }
}
