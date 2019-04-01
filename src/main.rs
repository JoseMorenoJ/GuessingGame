use std::io; //stdin()
use std::cmp::Ordering; //return type of cmp()
use rand::Rng; //thread_rng()

fn main() {
    println!("Guess the number!");

    //This will generate a random int between 0 and 99 included.
    let secret_number = rand::thread_rng().gen_range(0,100);
    //We can use std::dbg!() to 'debug' whatever is between the braces:
    //  this:    dbg!( rand::thread_rng().gen_range(0,100) )
    //  printed: [src\main.rs:9] rand::thread_rng().gen_range(0, 100) = 43

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //mut: mutable

        //read_line will return a Result type. This could be:
        //  Ok value: Just Ok, it continues.
        //  Err value: to handle errors.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); //If Ok value, continues. If Err, crashes.

        //Shadowing: You can declare a variable with the same name, it will overwrite it.
        //  trim(): erase the blanc spaces at the begining or end and the '\n' char.
        //  parse(): will return a number from a string type. 
        //      We must specify the type of the variable (u32) so the compiler knows.
        //      It also returns a Result type.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     //Result type = Ok type if ok.
            Err(_)=> continue,  //Result type = Err type if not ok.
        }; //we add the ';' or it complains. Don't understand it.

        println!("You guessed: {}", guess); //guess is printed where the {} are

        //Ordering is an enum type:
        //  You can only get one of the three values: Less, Greater, Equal.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
