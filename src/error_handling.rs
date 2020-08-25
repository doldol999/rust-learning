#[allow(dead_code)]
/**
 * Two categories of errors:
 * recoverable and unrecoverable
 */

pub mod error_handling {
  pub mod unrecoverable {
    pub fn execute() {
      //  panic!("crash and burn!");
      
      let v = vec![1, 2, 3];
      v[99];
    }
  }
  
  pub mod recoverable {
    use std::fs::File;

    pub fn execute() {
      let f = File::open("assets/file.txt"); // will succeed
      // let f = File::open("file.txt"); // will panic

      let f = match &f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
      };
      println!("{:?}", f);
    }
  }
 }