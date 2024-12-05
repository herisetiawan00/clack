use std::collections::HashMap;

use crate::presentation::screen::{self, Screen};

pub fn get_all() -> HashMap<String, Screen> {
    let mut routes = HashMap::new();

    routes.insert("/".to_string(), screen::landing::get());

    routes
}

pub fn get(name: &String) -> Screen {
    let routes = get_all();

    routes.get(name).unwrap().clone()
}
