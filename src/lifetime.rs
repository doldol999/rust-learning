pub mod lifetime {
  pub fn execute() {
    let f;
    
    { 
      // this is just an artificial scope 
      // to demonstrate lifetime
      let x = 12;

      // if written as f = &x 
      // compiler would complain about
      // `x`does not live long enough

      f = x; // this works because f here copies the value of x and is not referenced ( & )
    }

    println!("{}", f);
  }
}