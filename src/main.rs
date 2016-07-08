#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod command_parsing;
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod scene;
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod creature;
mod simulation;
extern crate rustc_serialize;
extern crate rand;



fn main() {
    println!("Hello, world!");
    simulation::Simulation::new().start();
}
