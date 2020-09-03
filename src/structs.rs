#[allow(dead_code)]
#[allow(unused_variables)]
pub mod structs {
    #[derive(Debug)] //this allows us to print structs contents
    pub struct User {
        /**
         * we used the owned String type rather than the &str string slice type. 
         * This is a deliberate choice because we want instances of this struct to own 
         * all of its data and for that data to be valid for as long as the entire struct is valid.
         */
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    /**
     * Another useful feature of impl blocks is that we’re allowed to define functions within impl blocks
     * that don’t take self as a parameter. These are called associated functions.
     * Associated functions are often used for constructors.
     * Also, each struct is allowed to have multiple impl blocks.
     */

    impl User {
        fn is_active(&self) -> bool {
            self.active
        }
    }

    pub fn execute() {
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        let user2 = build_user(String::from("test@gmail.com"), String::from("testUser"));

        let user3 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user2
        };

        /**
         * Using Tuple Structs without Named Fields to Create Different Types
         */
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        println!("Users: {:#?}, {:#?}, {:#?}", user1, user2, user3);

        println!("{}'s state is: {}", user1.email, user1.is_active()); // is_active is a method of User struct
        println!("{}'s state is: {}", user2.email, user2.is_active()); // is_active is a method of User struct
        println!("{}'s state is: {}", user3.email, user3.is_active()); // is_active is a method of User struct
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: false,
            sign_in_count: 1,
        }
    }
}