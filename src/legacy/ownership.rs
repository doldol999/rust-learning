#[allow(dead_code)]
pub mod ownership {
    /**
     * Reference via &
     * Dereference via *
     */

    pub fn execute() {
        let mut value = String::from("Hello"); // mutable String stored in heap
        println!("{}, Original string: {}", format_message(&value), value); // prints formatted string
        change(&mut value);
        println!("{}, Original string: {}", format_message(&value), value); // prints formatted string

        println!("{}", get_first_word(String::from("I am Number 4")));
    }

    /**
     * Below is a function that requires a referenced string (&str) type variable.
     * It also returns a String value containing "New Message: "
     * combined with the message String parameter.
     */

    fn format_message(message: &str) -> String {
        let mut new_message = String::from("New Message: ");
        new_message.push_str(message);
        new_message
    }

    /** 
     * Example of a mutable reference that starts with &mut
     * You can have only one mutable reference to a particular piece of data in a particular scope.
     * This code will fail:
     * 
     * let mut s = String::from("hello");
     * 
     * let r1 = &mut s;
     * let r2 = &mut s;
     * 
     * println!("{}, {}", r1, r2);
    */
    
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    fn get_first_word(some_string: String) -> String {
        // SLICING SAMPLE let slice = &some_string[0..2];
        let splits:Vec<&str>= some_string.split(" ").collect();
        return String::from(splits[0]);
    }
}