use std::{collections::HashMap, sync::LazyLock};

use chrono::NaiveDateTime;
use serenity::all::User;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Copy)]
pub enum ReminderFrequency {
    ThirtyMin,  // 30 min
    OneHour,    // 1h
    ThreeHours, // 3h
}

pub static REGISTRED_USERS: LazyLock<RwLock<HashMap<User, ReminderFrequency>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub static LAST_REMINDED_TIME: LazyLock<RwLock<HashMap<User, NaiveDateTime>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub async fn insert_new_user_to_remind(user: &User, frequency: ReminderFrequency) {
    REGISTRED_USERS
        .write()
        .await
        .insert(user.clone(), frequency);
    LAST_REMINDED_TIME
        .write()
        .await
        .insert(user.clone(), chrono::Utc::now().naive_utc());
}

pub async fn lookup_user_last_reminded_time(user: &User) -> Option<NaiveDateTime> {
    LAST_REMINDED_TIME.read().await.get(user).cloned()
}

pub async fn lookup_active_reminders_count() -> usize {
    REGISTRED_USERS.read().await.keys().count()
}
