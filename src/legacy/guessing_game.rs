#[allow(dead_code)]
pub mod guessing_game {
    use std::io;
    use std::cmp::Ordering;
    use rand::Rng;

    pub fn execute() {
        println!("Guess the number!");
    
        let secret_number = rand::thread_rng().gen_range(1, 101);
    
        let mut tries_count: u32 = 0;
    
        loop {
            println!("Enter your guess:");
    
            let mut guess = String::new();
    
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
    
            // Shadowing the string guess
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
            tries_count = tries_count + 1;
    
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("YOU WIN! The Secret Number is {}. Total number of tries: {}.", secret_number, tries_count);
                    break; // stops the loop
                },
            }
        }
    }
}

