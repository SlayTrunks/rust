use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the Number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut moves = 0;
    loop {
    
        println!("Pleasae input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error in something");
       //     .expect("Failed to read line");
        let guess:u32 = match guess.trim().parse(){ //parse returns Result enum with varient ok and
                                                    //err and err can be get with .expect too
            Ok(num)=>{
                moves+=1;
                num},
            Err(_) => continue,//_ is all value means for any type of error continue
        };
            //.expect("Please input the number");//u32 only contains
                                                                               //numeric data

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater=> println!("too big"),
            Ordering::Equal=> {
                println!("you win in {} moves",moves);
                    break;
            }
        }
    }
}
