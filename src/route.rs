use std::collections::HashMap;

use crate::presentation::screen;

pub fn get_all() -> HashMap<String, screen::Screen<'static>> {
    let mut routes = HashMap::new();

    routes.insert("/".to_string(), screen::landing::get());
    routes.insert("/home".to_string(), screen::home::get());

    routes
}

pub fn get(name: &String) -> screen::Screen<'static> {
    let routes = get_all();

    routes.get(name).unwrap().clone()
}
