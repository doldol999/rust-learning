mod menu;

use std::io::stdin;
use menu::{run, show_options};

fn main() {
  println!("Hello Rust World!"); // Chapter 1

  loop {
    let mut selection = String::from("");
    let options = show_options();

    println!("What would you like to try? {:?}", options);

    stdin()
      .read_line(&mut selection)
      .expect("Failed to read line");

    let selection = selection.trim();

    if options.contains(&selection) {
      run(selection);
    } else {
      if selection.is_empty() {
        println!("No selection found. Exiting program now.");
        break;
      } else {
        println!("Unknown option. Try again, or just hit ENTER if you want to exit.");
      }
    }
  }
}
