pub mod guessing_game {
  use std::io;
  use std::cmp::Ordering;
  use rand::Rng;

  #[allow(dead_code)] // trait needed to allow unused code
  pub fn play() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut tries = 0;

    println!("Let's Play Guessing Game!");

    loop {
      let mut guess = String::new();

      io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

      tries += 1;

      let guess: u32 = guess.trim().parse().expect("Please type a number!");

      match guess.cmp(&secret_number) {
        Ordering::Less => println!("{} is too small!", guess),
        Ordering::Greater => println!("{} is too big!", guess),
        Ordering::Equal => {
          println!("{} is correct! You had {} tries!", guess, tries);
          break;
        },
      }
    }

  }
}