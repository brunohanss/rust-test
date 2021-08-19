use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use std::cmp::Ordering;
use std::io;
mod endpoints;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(endpoints::hello)
            .service(endpoints::echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn guess_a_number() {
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
    }
}
