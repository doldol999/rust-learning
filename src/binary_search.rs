#[allow(dead_code)]
pub mod binary_search {
  use rand::Rng;

  fn largest(list: &[u32]) -> u32 {
    let mut largest = list[0];

    for &i in list {
      if i > largest { largest = i }
    }

    largest
  }

  fn binary_searched_largest(list: Vec<u32>) -> u32 {
    let mid = list.len() / 2;

    let first_half = list.split_at(mid).0;
    let second_half = list.split_at(mid).1;

    let f_largest = largest(first_half);
    let s_largest = largest(second_half);

    if f_largest > s_largest { return f_largest }

    return s_largest
  }

  fn linear_searched_largest(list: Vec<u32>) -> u32 {
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

    println!("The largest number in the list is {} with {} items.", linear_searched_largest(number_list), size);
  }
}