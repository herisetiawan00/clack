use crate::{
    entities::slack::authorization::Authorization, enums::user_mode::UserMode, states::State,
};

#[derive(Clone, PartialEq)]
pub struct Context {
    pub mode: UserMode,
    pub routes: Vec<String>,
    pub command: String,
    pub loading: bool,
    pub auth: Option<Authorization>,
    pub state: State,
    pub focus_id: String,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            mode: UserMode::Normal,
            command: String::default(),
            routes: vec![String::from("/")],
            loading: false,
            auth: None,
            state: State::new(),
            focus_id: String::new(),
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

    pub fn is_focus(&self, id: &String) -> bool {
        &self.focus_id == id
    }

    pub fn set_focus(&mut self, id: String) {
        self.focus_id = id;
    }
}
