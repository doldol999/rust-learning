#[cfg(test)]
mod tests {
  // Test for inequality use assert_eq! or assert_ne!
  // Use should_panic to check for panics

  #[test]
  fn it_works() {
      assert_eq!(2 + 2, 4); // check if value is equals to 4
  }

  #[test]
  #[should_panic] // Assuming this function would panic
  fn another() {
    panic!("Make this test fail");
  }
  
  #[test]
  fn name() {
    assert!(false) // checks if true or false
  }

  #[test]
  fn custom_message_test() {
    let message = String::from("Hello world");
    let slice = &message[0..1];
    assert!(
      slice == "e",
      "Slice's value is {} and not e",
      slice
    );
  }
}