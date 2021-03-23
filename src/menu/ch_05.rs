pub mod structs {

  #[derive(Debug)] // trait needed to print a struct
  struct User {
    f_name: String,
    m_name: String,
    l_name: String,
  }

  impl User { // sample implementation of a method in structs
    fn full_name(&self) -> String {
      let mut result = String::from(&self.f_name); // declare result with f_name as its initial content
      // Next, add the remaining content with white spaces in between them
      result.push_str(" ");
      result.push_str(&self.m_name);
      result.push_str(" ");
      result.push_str(&self.l_name);
      // return the result value
      result
    }

    fn new(f_name: String, m_name: String, l_name: String) -> User { // Associated function in struct used as a constructor
      User {
        f_name,
        m_name,
        l_name,
      }
    }
  }

  pub fn execute() {
    let user1 = User::new(
      String::from("Mark"),    // first name
      String::from("Ballega"), // middle name
      String::from("Lumbao"),  // last name
    );

    println!("name: {} from object: {:?}", user1.full_name(), user1); // use {:?} to debug/print a struct value
  }
}