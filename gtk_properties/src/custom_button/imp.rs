use std::cell::Cell;

use glib::{BindingFlags, ParamSpec, ParamSpecInt, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

#[derive(Default)]
pub struct CustomButton{
    number: Cell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "GTKCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for CustomButton {
    fn properties() -> &'static [ParamSpec]  {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecInt::builder("number").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "number" => {
                let input_number =
                    value.get().expect("The value needs to be of type 'i32'.");
                self.number.replace(input_number);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) ->  Value {
        match pspec.name() {
            "number" => self.number.get().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.bind_property("number", obj.as_ref(), "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
    }
}

impl WidgetImpl for CustomButton {}

impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        let incremented_number = self.number.get() + 1;
        self.obj().set_property("number", &incremented_number);
    }
}
