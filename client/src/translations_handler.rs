use std::collections::{HashMap, HashSet};

use crate::{utils, AppContext};

pub enum Translations {
    English,
    Polish,
}

pub fn get_translations(profile: Translations) -> HashMap<String, String> {
    let tokens = vec![
        "menu.title",
        "menu.instructions.1",
        "menu.instructions.2",
        "menu.instructions.3",
        "menu.instructions.q",
        "rabbitmq.title",
        "rabbitmq.data.workers",
        "rabbitmq.data.tasks",
        "logs.sent.title",
        "logs.received.title",
    ];

    let translations = match profile {
        Translations::Polish => tokens
            .iter()
            .map(|token| (token.to_string(), get_property("pl.", token)))
            .collect(),
        _ => tokens
            .iter()
            .map(|token| (token.to_string(), get_property("en.", token)))
            .collect(),
    };

    translations
}

fn get_property(prefix: &str, token: &str) -> String {
    let key = &format!("{}{}", prefix, token);
    utils::get_env_var(key, key, true)
}

pub fn load_translation(key: &str, app_context: &AppContext) -> String {
    app_context
        .translations
        .get(key)
        .unwrap_or(&key.to_string())
        .clone()
}
