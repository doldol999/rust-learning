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
    use std::io::ErrorKind;

    pub fn execute() {
      let f = File::open("assets/file.txt"); // will succeed
      // let f = File::open("file.txt"); // will panic

      // grabs file from path then panics if file is not found
      let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
      };

      println!("{:?}", &f);


      // grabs file from path
      // creates file in not existing
      // panics if file creation fails

      let f = File::open("assets/file2.txt"); // will succeed

      let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
          ErrorKind::NotFound => match File::create("assets/file2.txt") {
            Ok(fc) => fc,
            Err(err) => panic!("Problem creating the file: {:?}", err),
          },
          other_error => panic!("Problem opening the file: {:?}", other_error)
        },
      };

      println!("{:?}", &f);

      // Same concept with the method above
      // except it uses closures, not match operator

      let f = File::open("assets/hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
      });

      println!("{:?}", &f);

      println!("{:?}", File::open("assets/file2.txt").unwrap()); // This will either return a file or a !panic
      println!("{:?}", File::open("assets/file3.txt").expect("Failed to load file.")); // Same with ^ but adds in a custom error message
    }

    pub fn execute_error_propagation() {
      
    }
  }
 }