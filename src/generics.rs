#[allow(dead_code)]
pub mod generics {
  use rand::Rng;

  fn largest(list: Vec<u32>) -> u32 {
    let mut largest = list[0];

    for i in list {
      if i > largest { largest = i }
    }

    largest
  }

  pub fn execute() {
    let size = rand::thread_rng().gen_range(1, 100);

    let mut number_list: Vec<u32> = Vec::new();

    for _ in 0..size {
      number_list.push(rand::thread_rng().gen_range(1, 100))
    }

    println!("The largest number in the list is {} with {} items.", largest(number_list), size);
  }
}