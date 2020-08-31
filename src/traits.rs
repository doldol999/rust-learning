#[allow(dead_code)]
// a trait example
pub trait Summary {
  fn summarize(&self) -> String; // lets developers define the behavior

  fn summarize_name(&self) -> String; // lets developers define the value to be used in def_summarize below
  
  fn def_summarize(&self) -> String { // implemets default behavior
    format!("(Discover more about...) {}.", self.summarize_name())
  }

  fn summarize_age(&self) -> u8;

  fn age_suffix(&self) -> String;
}

pub mod traits {
  use rand::Rng;
  use std::fmt;
  use std::fmt::Display;
  use crate::traits::Summary;

  struct People {
    name: String,
    age: u8,
  }
  
  // implemented Display trait for the People struct
  impl Display for People {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Write strictly the first element into the supplied output
      // stream: `f`. Returns `fmt::Result` which indicates whether the
      // operation succeeded or failed. Note that `write!` uses syntax which
      // is very similar to `println!`.
      write!(f, "{}", self)
    }
  }
  
  impl People {
    fn new(name: String, age: u8) -> People {
      People {
        name,
        age
      }
    }

    fn get_age(&self) -> u8 {
      self.age
    }
  }
  
  // here is how you would implement a trait to a struct
  impl Summary for People {
    fn summarize(&self) -> String {
      String::from(format!("{} is {} years old.", self.name, self.get_age()))
    }
    
    fn summarize_name(&self) -> String {
      String::from(&self.name)
    }

    fn summarize_age(&self) -> u8 {
      self.age
    }

    fn age_suffix(&self) -> String {
      if self.age > 1 {
        return String::from("th")
      }

      String::from("st")
    }
  }
  
  impl People {
    // implements trait as a parameter
    fn notify_birthday(notif: &(impl Summary + Display)) {
      println!("Happy {}{} birthday {}", notif.summarize_age(), notif.age_suffix(), notif.summarize_name());
    }
  }

  // a useful way of using Traits in a generic function
  // this is an example of a Trait Bound Syntax
  fn display<T: Display>(item: T) {
    println!("{}", item);
  }

  pub fn execute() {
    let mut person = People::new(String::from("Dextre Linus Lumbao"), rand::thread_rng().gen_range(0, 3));
    person.name = String::from("Dextre Linus Cadiz Lumbao");
    
    println!("{}", person.def_summarize());
    println!("{}", person.summarize());

    People::notify_birthday(&person);

    display(100);
    display("item: T");
  }
}