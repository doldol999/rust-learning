mod variables;
mod guessing_game;
mod ownership;
mod structs;
mod enums;
mod collections;
mod error_handling;
mod generics;
mod binary_search;
mod traits;
mod lifetime;

#[allow(unused_imports)]
use enums::enums::execute as executeEnums; // using as keyword to replace execute with executeEnums

#[allow(unused_imports)]
use collections::collections::execute_vector as vector;

fn main() {
    // variables::variables::execute();
    // guessing_game::guessing_game::execute();
    // ownership::ownership::execute();
    // structs::structs::execute();

    // enums::enums::execute(); // relative path
    // crate::enums::enums::execute(); // absolute path
    // executeEnums(); // using use keyword

    // vector(); // runst vector demos from collections module
    // collections::collections::execute_string();
    // collections::collections::execute_hash_maps();

    // error_handling::error_handling::unrecoverable::execute();
    // error_handling::error_handling::recoverable::execute();
    // error_handling::error_handling::recoverable::execute_error_propagation();

    // binary_search::binary_search::execute();
    
    // generics::generics::execute();

    // traits::traits::execute();

    lifetime::lifetime::execute();
}
