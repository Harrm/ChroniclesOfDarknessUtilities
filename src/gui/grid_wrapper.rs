use super::gtk;
use super::gtk::{WidgetExt, ScrolledWindowExt, Inhibit, Cast};
use std::collections::HashMap;


pub struct GridWithLabels;



impl GridWithLabels {
    pub fn setContent<Widget, Content>
                     (grid: &mut gtk::Grid, 
                      map: &HashMap<String, Content>,
                      action: &Fn(Option<Widget>, &Content)) 
        where Widget: gtk::IsA<gtk::Widget> {
        let mut i = 0;
        while let Some(label) = 
            Self::getWidgetFromGrid::<gtk::Label>(grid, 0, i) {
            if let Some(val) = map.get(&label.get_label().unwrap()) {
                let secondWidget = 
                    Self::getWidgetFromGrid::<Widget>(grid, 1, i);
                action(secondWidget, val);
            }
    
            i+=1;
        }
    }

    fn getWidgetFromGrid<T>(grid: &gtk::Grid, x: i32, y: i32) -> Option<T>
        where T: gtk::IsA<gtk::Widget> {
        let cast_result = grid.get_child_at(x, y)
                              .map(|w| w.downcast::<T>());
        if let Some(Ok(wgt)) = cast_result {
            Some(wgt)
        } else {
            None
        }
    }
}