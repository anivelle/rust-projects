use gtk::cairo;
use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use once_cell::sync::Lazy;

const APP_ID: &str = "org.gtk_rs.cairo_practice";
// Track window width and height (assumes window is never resized please never resize it outside of
// these variables dear god)
const WIDTH: i32 = 20 * 64;
const HEIGHT: i32 = 20 * 32;
// Static Cairo region to make XORing pixels for display a lot simpler
static mut REGION: Lazy<cairo::Region> = Lazy::new(|| {
    let region = cairo::Region::create_rectangle(&cairo::RectangleInt::new(0, 0, WIDTH, HEIGHT));
    region
});
static mut X: i32 = 0;
static mut Y: i32 = 0;

fn draw_func(area: &DrawingArea, cr: &cairo::Context, width: i32, height: i32) {
    let context = area.style_context();
    let pixel_width = width / 64;
    let pixel_height = height / 32;
    let rect =
        cairo::Region::create_rectangle(&cairo::RectangleInt::new(0, 0, pixel_width, pixel_height));
    unsafe {
        REGION.xor(&rect).unwrap();
        if X == 64 {
            X = 0;
        } else if X == -1 {
            X = 63;
        }
        if Y == 32 {
            Y = 0;
        } else if Y == -1 {
            Y = 31;
        }
        rect.translate(
            rect.rectangle(0).width() * X,
            rect.rectangle(0).height() * Y,
        );
        REGION.xor(&rect).unwrap();
    }
    let color = context.color();
    unsafe {
        cr.add_region(&REGION);
    }
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
        .width_request(WIDTH)
        .height_request(HEIGHT)
        .build();

    darea.set_draw_func(draw_func);

    // REPEATED EVENTS LETS GO
    // gtk::glib::timeout_add(interval, func)
    darea.queue_draw();
    // Create the application window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&darea)
        .build();
    // Doesn't get events when the parent is the drawing area
    let key = gtk::EventControllerKey::new();
    // TODO: Figure out how to redraw the drawing area
    key.connect_key_released(|key, keyval, _, _| {
        println!("Key event captured");
        // Move the little square around the screen hehe
        match keyval {
            gdk::Key::Left => unsafe {
                X -= 1;
            },
            gdk::Key::Right => unsafe {
                X += 1;
            },
            gdk::Key::Up => unsafe {
                Y -= 1;
            },
            gdk::Key::Down => unsafe {
                Y += 1;
            },
            _ => todo!(),
        }
        // Doesn't redraw drawing area, as you'd expect, but no parent exists
        key.widget().queue_draw();
    });
    //darea.add_controller(&key);
    window.add_controller(&key);


    // Present window
    window.show();
    //darea.grab_focus();
}
