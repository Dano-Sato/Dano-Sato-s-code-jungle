use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    {
        let secret_number = rand::thread_rng().gen_range(1,101); 

        println!("The secret number is: {}",secret_number);


        //infinite loop starts at this point.
        //You can get out of the loop by "break" keyword.

        loop{

        println!("Guess the number!");
        println!("Please input your guess.");

        let mut guess = String::new();
        

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        //.expect method take care of the error from Type: Result
        //.expect unwraps the Type: Result
        // in a lot of cases, when functions in Rust process data, 
        //they wrap the result... That must be evaluated properly.

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) =>num,
            Err(_) => continue,
        };
        //Error handling with match expression.
        //match expression could return value,and the value could be assigned to variable.
    


        println!("You guessed: {}", guess);    

            match guess.cmp(&secret_number)
            {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                break;
                },
    
            }
    
        }
    }
}
