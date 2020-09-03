pub mod lifetime {
  // Lifetime annotations
  // &i32        // a reference
  // &'a i32     // a reference with an explicit lifetime
  // &'a mut i32 // a mutable reference with an explicit lifetime

  // Lifetime annotations in function signatures
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
      return x;
    } else {
      return y;
    }
  }

  
  pub fn execute() {
    let f;
    let outer_str = "hello";
    let largest_str: &str;

    { 
      // this is just an artificial scope 
      // to demonstrate lifetime
      let x = 12;

      // if written as f = &x 
      // compiler would complain about
      // `x`does not live long enough

      f = x; // this works because f here copies the value of x and is not referenced ( & )
    
      let inner_str = "world";
  
      largest_str = longest(&inner_str, &outer_str);
    }
    
    println!("{}", f);
    println!("{}", largest_str);
  }
}