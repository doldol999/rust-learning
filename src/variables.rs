pub mod variables {
    pub fn execute() {
        println!("VARIABLES");

        /* @NOTE:
        For integers size like i8 ( 8-bit )
        Sizes are being computed based on Two's compliment
        Sample i8 is from -(2 ^ n-1) to (2 ^ n-1) - 1
        Which means i8 is from -128 to 127
        */

        let x: i8 = -5; // signed can have +/- signs

        let y: u8 = 5; // unsigned are just positive ints

        let z: f32 = 5.1; // floating points comes with decimal points

        const CONSTANT_STRING: &str = "    ";

        println!("signed integer: {}, unsigned integer: {}, floating point: {}.", x, y, z);

        println!("Length of CONSTANT_STRING: {}", CONSTANT_STRING.len());

        /* @NOTE
        Tupples
        let tup_sample: (char, u8, f32) = ('a', 5, 5.1);
        */

        println!("Remainder: {}", get_remainder(10, 7));

        // Sample loop
        for num in (1..10).rev() {
            print!("{} ", num);
        }
    }

    // @NOTE expression sample
    fn get_remainder(n1: i8, n2: i8) -> i8 {
        return n1 % n2;
        // or n1 % n2
    }
}