use std::{collections::HashMap, sync::LazyLock};

use chrono::{NaiveDateTime, TimeDelta};
use serenity::all::User;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Copy)]
pub enum ReminderFrequency {
    ThirtyMin,  // 30 min
    OneHour,    // 1h
    ThreeHours, // 3h
}

impl From<ReminderFrequency> for TimeDelta {
    fn from(value: ReminderFrequency) -> Self {
        match value {
            ReminderFrequency::ThirtyMin => TimeDelta::seconds(3),
            ReminderFrequency::OneHour => TimeDelta::minutes(60),
            ReminderFrequency::ThreeHours => TimeDelta::minutes(3 * 60),
        }
    }
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

pub async fn lookup_active_reminders_count() -> usize {
    REGISTRED_USERS.read().await.keys().count()
}

pub async fn update_user_to_reminder(user: &User, date: NaiveDateTime) {
    LAST_REMINDED_TIME.write().await.insert(user.clone(), date);
}
