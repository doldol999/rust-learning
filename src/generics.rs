#[allow(dead_code)]
pub mod generics {
  use rand::Rng;

  #[derive(Debug)]
  struct People<T> {
    name: String,
    age: T,
  }

  impl<T> People<T> {
    fn new(name: String, age: T) -> People<T> {
      People {
        name,
        age
      }
    }
  }

  impl<T> People<T> {
    fn get_age(&self) -> &T {
      &self.age
    }
  }

  pub fn execute() {
    let mut person = People::new(String::from("Dextre Linus Lumbao"), rand::thread_rng().gen_range(0, 3));
    person.name = String::from("Dextre Linus Cadiz Lumbao");
    println!("{}, {}", person.name, person.age);
  }
}