pub mod guessing_game {
  use std::io;
  use std::cmp::Ordering;
  use rand::Rng;

  struct Guess {
    value: i32,
  }

  impl Guess {
    pub fn new(value: i32) -> Guess {
      if value < 1 || value > 100 {
        panic!("You can only guess numbers from 1 to 100");
      }

      Guess { value }
    }

    pub fn value(&self) -> i32 {
      self.value
    }
  }

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

      let guess: Guess = Guess::new(guess.trim().parse().expect("Please type a number!"));

      match guess.value().cmp(&secret_number) {
        Ordering::Less => println!("{} is too small!", guess.value()),
        Ordering::Greater => println!("{} is too big!", guess.value()),
        Ordering::Equal => {
          println!("{} is correct! You had {} tries!", guess.value(), tries);
          break;
        },
      }
    }

  }
}