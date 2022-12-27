mod custom_button;

use custom_button::CustomButton;
use glib::BindingFlags;
use gtk::prelude::*;
use gtk::{glib, Align, Application, ApplicationWindow, Box, Orientation, Switch};

const APP_ID: &str = "org.gtk_rs.GObjectProperties1";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let button_1 = CustomButton::new();
    let button_2 = CustomButton::new();

    button_1
        .bind_property("number", &button_2, "number")
        .transform_to(|_, number: i32| {
            let incremented_number = number + 1;
            Some(incremented_number.to_value())
        })
        .transform_from(|_, number: i32| {
            let decremented_number = number - 1;
            Some(decremented_number.to_value())
        })
        .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
        .build();
    let gtk_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_1);
    gtk_box.append(&button_2);

    button_1.connect_notify_local(Some("number"), move |button, _| {
        let number = button.property::<i32>("number");
        println!("The current number of 'button_1' is {}.", number);
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    window.present();
}
