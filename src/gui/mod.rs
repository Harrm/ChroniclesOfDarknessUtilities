extern crate gtk;



pub fn init() -> Result<(), &'static str> {
    gtk::init().map_err(|_| "Error when initializing gtk")
}



pub fn run() {
    gtk::main();
}


mod grid_wrapper;
mod main_window;
pub use self::main_window::Window;
