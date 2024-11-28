use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    pub with_default_config: bool,
    pub keymaps: KeyMaps,
    pub slack: Slack,
    pub status_line: StatusLine,
    pub appearance: Appearance,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KeyMaps {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
    pub exit: String,
    pub open: String,
    pub search: String,
    pub interact: String,
    pub send: String,
    pub focus: KeyMapsFocus,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KeyMapsFocus {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Slack {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatusLine {
    pub left: StatusLineSide,
    pub right: StatusLineSide,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatusLineSide {
    pub primary: String,
    pub secondary: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Appearance {
    pub left_separator: String,
    pub right_separator: String,
    pub separator: String,
}

impl Configuration {
    pub fn merge_with(&self, other: PartialConfiguration) -> Configuration {
        Configuration {
            with_default_config: other
                .with_default_config
                .unwrap_or(self.with_default_config),
            keymaps: other
                .keymaps
                .map_or(self.keymaps.clone(), |keymaps| KeyMaps {
                    up: keymaps.up.unwrap_or(self.keymaps.up.clone()),
                    down: keymaps.down.unwrap_or(self.keymaps.down.clone()),
                    left: keymaps.left.unwrap_or(self.keymaps.left.clone()),
                    right: keymaps.right.unwrap_or(self.keymaps.right.clone()),
                    exit: keymaps.exit.unwrap_or(self.keymaps.exit.clone()),
                    open: keymaps.open.unwrap_or(self.keymaps.open.clone()),
                    search: keymaps.search.unwrap_or(self.keymaps.search.clone()),
                    interact: keymaps.interact.unwrap_or(self.keymaps.interact.clone()),
                    send: keymaps.send.unwrap_or(self.keymaps.send.clone()),
                    focus: keymaps
                        .focus
                        .map_or(self.keymaps.focus.clone(), |focus| KeyMapsFocus {
                            up: focus.up.unwrap_or(self.keymaps.focus.up.clone()),
                            down: focus.down.unwrap_or(self.keymaps.focus.down.clone()),
                            left: focus.left.unwrap_or(self.keymaps.focus.left.clone()),
                            right: focus.right.unwrap_or(self.keymaps.focus.right.clone()),
                        }),
                }),
            slack: other.slack.map_or(self.slack.clone(), |slack| Slack {
                client_id: slack.client_id.unwrap_or(self.slack.client_id.clone()),
                client_secret: slack
                    .client_secret
                    .unwrap_or(self.slack.client_secret.clone()),
            }),
            status_line: other
                .status_line
                .map_or(self.status_line.clone(), |status_line| StatusLine {
                    left: status_line
                        .left
                        .map_or(self.status_line.left.clone(), |left| StatusLineSide {
                            primary: left
                                .primary
                                .unwrap_or(self.status_line.left.primary.clone()),
                            secondary: left
                                .secondary
                                .unwrap_or(self.status_line.left.secondary.clone()),
                        }),
                    right: status_line
                        .right
                        .map_or(self.status_line.right.clone(), |right| StatusLineSide {
                            primary: right
                                .primary
                                .unwrap_or(self.status_line.right.primary.clone()),
                            secondary: right
                                .secondary
                                .unwrap_or(self.status_line.right.secondary.clone()),
                        }),
                }),
            appearance: other
                .appearance
                .map_or(self.appearance.clone(), |appearance| Appearance {
                    left_separator: appearance
                        .left_separator
                        .unwrap_or(self.appearance.left_separator.clone()),
                    right_separator: appearance
                        .right_separator
                        .unwrap_or(self.appearance.right_separator.clone()),
                    separator: appearance
                        .separator
                        .unwrap_or(self.appearance.separator.clone()),
                }),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PartialConfiguration {
    pub with_default_config: Option<bool>,
    pub keymaps: Option<PartialKeyMaps>,
    pub slack: Option<PartialSlack>,
    pub status_line: Option<PartialStatusLine>,
    pub appearance: Option<PartialAppearance>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PartialKeyMaps {
    pub up: Option<String>,
    pub down: Option<String>,
    pub left: Option<String>,
    pub right: Option<String>,
    pub exit: Option<String>,
    pub open: Option<String>,
    pub search: Option<String>,
    pub interact: Option<String>,
    pub send: Option<String>,
    pub focus: Option<PartialKeyMapsFocus>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PartialKeyMapsFocus {
    pub up: Option<String>,
    pub down: Option<String>,
    pub left: Option<String>,
    pub right: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PartialSlack {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PartialStatusLine {
    pub left: Option<PartialStatusLineSide>,
    pub right: Option<PartialStatusLineSide>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PartialStatusLineSide {
    pub primary: Option<String>,
    pub secondary: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PartialAppearance {
    left_separator: Option<String>,
    right_separator: Option<String>,
    separator: Option<String>,
}

impl PartialConfiguration {
    pub fn empty() -> PartialConfiguration {
        PartialConfiguration {
            with_default_config: None,
            keymaps: None,
            slack: None,
            status_line: None,
            appearance: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.with_default_config == None
            && self.keymaps == None
            && self.slack == None
            && self.status_line == None
    }

    pub fn unwrap_all(&self) -> Configuration {
        Configuration {
            with_default_config: self.with_default_config.unwrap(),
            keymaps: self
                .keymaps
                .clone()
                .map(|keymaps| KeyMaps {
                    up: keymaps.up.unwrap(),
                    down: keymaps.down.unwrap(),
                    left: keymaps.left.unwrap(),
                    right: keymaps.right.unwrap(),
                    exit: keymaps.exit.unwrap(),
                    open: keymaps.open.unwrap(),
                    search: keymaps.search.unwrap(),
                    interact: keymaps.interact.unwrap(),
                    send: keymaps.send.unwrap(),
                    focus: keymaps
                        .focus
                        .clone()
                        .map(|focus| KeyMapsFocus {
                            up: focus.up.unwrap(),
                            down: focus.down.unwrap(),
                            left: focus.left.unwrap(),
                            right: focus.right.unwrap(),
                        })
                        .unwrap(),
                })
                .unwrap(),
            slack: self
                .slack
                .clone()
                .map(|slack| Slack {
                    client_id: slack.client_id.unwrap(),
                    client_secret: slack.client_secret.unwrap(),
                })
                .unwrap(),
            status_line: self
                .status_line
                .clone()
                .map(|status_line| StatusLine {
                    left: status_line
                        .left
                        .clone()
                        .map(|left| StatusLineSide {
                            primary: left.primary.unwrap(),
                            secondary: left.secondary.unwrap(),
                        })
                        .unwrap(),
                    right: status_line
                        .right
                        .clone()
                        .map(|right| StatusLineSide {
                            primary: right.primary.unwrap(),
                            secondary: right.secondary.unwrap(),
                        })
                        .unwrap(),
                })
                .unwrap(),
            appearance: self
                .appearance
                .clone()
                .map(|appearance| Appearance {
                    left_separator: appearance.left_separator.unwrap(),
                    right_separator: appearance.right_separator.unwrap(),
                    separator: appearance.separator.unwrap(),
                })
                .unwrap(),
        }
    }
}
