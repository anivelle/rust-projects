use gtk::prelude::*;


fn main() {
    let string_some_value = "Hello!".to_value();
    let string_none_value = None::<String>.to_value();

    let string_some = string_some_value
        .get::<Option<String>>()
        .expect("The value needs to be of type 'Option<String>'.");

    let string_none = string_none_value
        .get::<Option<String>>()
        .expect("The value needs to be of type 'Option<String>'.");

    assert_eq!(string_some, Some("Hello!".to_string()));
    assert_eq!(string_none, None);
}
