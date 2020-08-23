#[allow(dead_code)]
pub mod enums {
  #[derive(Debug)]
  enum Cakes {
    Chocolate,
    Vanilla,
    Yam,
  }

  pub fn execute() {
    let chocolate_cake = Cakes::Chocolate;
    let vanilla_cake = Cakes::Vanilla;
    let yam_cake = Cakes::Yam;

    let option1 = Some(&chocolate_cake);
    let option2 = Some(&vanilla_cake);
    let option3 = Some(&yam_cake);
    
    println!("{:?}", &chocolate_cake);
    println!("{:?}", &vanilla_cake);
    println!("{:?}", &yam_cake);

    // match operator considers all of the options for an enum
    fn get_cake_flavor(cake: Option<&Cakes>) -> String {
      match cake {
        Some(Cakes::Chocolate) => String::from("This is a chocolate cake"),
        Some(Cakes::Vanilla) => String::from("This is a vanilla cake"),
        Some(Cakes::Yam) => String::from("This is a yam cake"),
        _ => String::from("This flavor is unknown to us"), // none
      }
    }

    // if let operator only checks for one enum option
    fn is_chocolate(cake: Option<&Cakes>) -> bool {
      if let Some(Cakes::Chocolate) = cake { true } else { false }
    }

    println!("{}", get_cake_flavor(option1));
    println!("{}", get_cake_flavor(option2));
    println!("{}", get_cake_flavor(option3));
    
    // unwrap returns the contained value of Some
    println!("Is {:?} a chocolate cake?: {:?}", option1.unwrap(), is_chocolate(option1));
    println!("Is {:?} a chocolate cake?: {:?}", option2.unwrap(), is_chocolate(option2));
    println!("Is {:?} a chocolate cake?: {:?}", option3.unwrap(), is_chocolate(option3));
  }
}
