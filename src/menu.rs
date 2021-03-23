mod ch_02;
mod ch_05;
mod ch_07;
mod ch_08;

use ch_02::guessing_game::play as demo_guessing_game;
use ch_05::structs::execute as demo_structs;
use ch_07::separate_module::execute as demo_separate_module;
use ch_08::vectors::demo as demo_vectors;

pub fn show_options() -> Vec<&'static str> {
  vec!["guessing game", "structs", "modules", "vectors"]
}

pub fn run (selection: &str) {
  if selection == "guessing game" {
    demo_guessing_game(); // Executes Chapter 2 Guessing Game
  } else if selection == "structs" {
    demo_structs();
  } else if selection == "modules" {
    demo_separate_module(); // Demo for extracting scripts and modules
  } else if selection == "vectors" {
    demo_vectors();
  }
}