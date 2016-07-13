use super::gtk;
use super::gtk::{WidgetExt, ScrolledWindowExt, Inhibit, Cast};
use std::collections::HashMap;
use super::grid_wrapper::GridWithLabels;



pub struct Window {
    builder: gtk::Builder
}



impl Window {
    pub fn new(layout_file: &str, attributes_names: &[&str], 
                                  skills_names: &[&str]) -> Window {
        let builder = gtk::Builder::new();
        if let Err(err) = builder.add_from_file(layout_file) {
            println!("Error reading layout file: {}", err);
        }
        let window: gtk::Window = builder.get_object("application_window").unwrap();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let mut attr_grid: gtk::Grid = 
            builder.get_object("attributes_grid").unwrap();
        for (i, name) in attributes_names.iter().enumerate() {
            let label = gtk::Label::new_with_mnemonic(Some(name));
            let button = gtk::SpinButton::new_with_range(1f64, 10f64, 1f64);
            attr_grid.attach(&label, 0, i as i32, 1, 1);
            attr_grid.attach(&button, 1, i as i32, 1, 1);
        }
        let skills_grid: gtk::Grid = builder.get_object("skills_grid").unwrap();
        for (i, name) in skills_names.iter().enumerate() {
            let label = gtk::Label::new_with_mnemonic(Some(name));
            let button = gtk::SpinButton::new_with_range(0f64, 10f64, 1f64);
            skills_grid.attach(&label, 0, i as i32, 1, 1);
            skills_grid.attach(&button, 1, i as i32, 1, 1);
        }
        let advantages_grid: gtk::Grid = 
            builder.get_object("advantages_grid").unwrap();
        for (i, advantage)
        window.show_all();

        Window {builder: builder}
    }



    pub fn setAttributes(&mut self, attributes: &HashMap<String, u8>) {
        let mut grid: gtk::Grid = 
            self.builder.get_object("attributes_grid").unwrap();
        GridWithLabels::setContent::<gtk::SpinButton, u8>
           (&mut grid, attributes,
            &|button: Option<gtk::SpinButton>, value| {
                button.unwrap().set_value(*value as f64);
            });
    }



    pub fn setSkills(&mut self, skills: &HashMap<String, u8>) {
        let mut grid: gtk::Grid = 
            self.builder.get_object("skills_grid").unwrap();
        GridWithLabels::setContent::<gtk::SpinButton, u8>
           (&mut grid, skills,
            &|button: Option<gtk::SpinButton>, value| {
                button.unwrap().set_value(*value as f64);
            });
    }
}