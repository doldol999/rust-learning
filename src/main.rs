mod variables;
mod guessing_game;
mod ownership;
mod structs;
mod enums;
mod collections;

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
    collections::collections::execute_string();
}
