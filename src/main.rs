
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod command_parser;
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod scene;
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod creature;
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod attribute;
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod health;
mod simulation;



fn main() {
    println!("Hello, world!");
    simulation::Simulation::new().start();
}
