use std::{
    collections::HashMap,
    sync::LazyLock,
};

use chrono::NaiveDateTime;
use serenity::all::User;
use tokio::sync::RwLock;

pub static REGISTERED_USERS: LazyLock<RwLock<HashMap<User, NaiveDateTime>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub async fn insert_new_user_to_remind(user: &User) {
    REGISTERED_USERS
        .write()
        .await
        .insert(user.clone(), chrono::Utc::now().naive_utc());
}

pub async fn lookup_user_last_reminded_time(user: &User) -> Option<NaiveDateTime> {
    REGISTERED_USERS.read().await.get(user).cloned()
}

pub async fn lookup_active_reminders_count() -> usize {
    REGISTERED_USERS.read().await.keys().count()
}
