use gtk::cairo;
use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn draw_func(area: &DrawingArea, cr: &cairo::Context, width: i32, height: i32) {
    println!("{} {}", width, height);
    let context = area.style_context();
    let rect = cairo::Region::create_rectangle(&cairo::RectangleInt::new(20, 20, 100, 100));
    let color = context.color();
    cr.arc(
        width as f64 / 2.0,
        height as f64 / 2.0,
        if width > height {
            height as f64 / 2.0
        } else {
            width as f64 / 2.0
        },
        0.0,
        2.0 * 3.1415926535,
    );
    cr.set_source_rgba(
        color.red().into(),
        color.green().into(),
        color.blue().into(),
        color.alpha().into(),
    );
    cr.fill().expect("Could not fill context");
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let darea = DrawingArea::builder()
        .content_width(400)
        .content_height(400)
        .build();

    darea.set_draw_func(draw_func);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&darea)
        .build();

    // Present window
    window.show();
    darea.queue_draw();
}
