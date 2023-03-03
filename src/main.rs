use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess Thine Number");
    println!("Please input thine guess:");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    // println!("Psst..The secret number is {secret_num}");
    loop {
        
    
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess )
    .expect("Alas! I hath failed to read thine guess!");

    let guess: u32  = match guess.trim().parse(){
     Ok(num) => num,
     Err(_) => {
            println!("This isnt a number, though shall try again!");
            continue;
        }
     };


    println!("Thine guess is: {guess}");

    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Thou shalt guess higher"),
        Ordering::Greater => println!("Thoug shalt guess lower"),
        Ordering::Equal => {
            println!("THINE HAS DONE IT! {guess} WAS THE NUMBER!");
            break;
        }
    }
}
}
