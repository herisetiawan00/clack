#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub routes: Vec<String>,
    pub command: String,
    pub loading: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            command: String::default(),
            routes: vec![String::from("/")],
            loading: false,
        }
    }
}

impl Context {
    pub fn current_route(&self) -> &String {
        self.routes.last().unwrap()
    }

    pub fn clear_command(&mut self) {
        self.command = String::new();
    }

    pub fn route_push(&mut self, route: String) {
        self.routes.push(route);
    }

    pub fn route_pop(&mut self) {
        self.routes.pop();
    }

    pub fn is_exit(&self) -> bool {
        self.routes.is_empty()
    }

    pub fn show_loading(&mut self) {
        self.loading = true;
    }

    pub fn hide_loading(&mut self) {
        self.loading = false;
    }
}
