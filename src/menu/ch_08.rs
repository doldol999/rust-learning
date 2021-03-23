pub mod vectors {
  /**
   * Vec<T>
   * Vectors can only store values of the same type
   */
  pub fn demo() {
    let number_prefixes: Vec<u16> = vec![0917, 0909, 0906, 0930];

    for prefix in number_prefixes {
      println!("{}", prefix);
    }
  }
}