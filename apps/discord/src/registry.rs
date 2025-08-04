use std::{collections::HashMap, sync::LazyLock};

use chrono::{NaiveDateTime, TimeDelta};
use serenity::all::{CacheHttp, User, UserId};
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::database;

#[derive(Debug, Clone, Copy)]
pub enum ReminderFrequency {
    ThirtyMin,  // 30 min
    OneHour,    // 1h
    ThreeHours, // 3h
}

impl From<ReminderFrequency> for TimeDelta {
    fn from(value: ReminderFrequency) -> Self {
        match value {
            ReminderFrequency::ThirtyMin => TimeDelta::seconds(5),
            ReminderFrequency::OneHour => TimeDelta::hours(1),
            ReminderFrequency::ThreeHours => TimeDelta::hours(3),
        }
    }
}

pub static REGISTRED_USERS: LazyLock<RwLock<HashMap<User, ReminderFrequency>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub static LAST_REMINDED_TIME: LazyLock<RwLock<HashMap<User, NaiveDateTime>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub async fn insert_new_user_to_remind(user: &User, frequency: ReminderFrequency) {
    let now = chrono::Utc::now().naive_utc();

    REGISTRED_USERS
        .write()
        .await
        .insert(user.clone(), frequency);
    info!("Inserted {} in registred users", user.name);

    LAST_REMINDED_TIME.write().await.insert(user.clone(), now);
    info!("Inserted last updated time for user {} as now", user.name);

    // Save to database
    if let Err(e) = database::save_user_reminder(user, frequency, now).await {
        error!("Failed to save user {} to database: {}", user.name, e);
    }
}

pub async fn lookup_active_reminders_count() -> usize {
    REGISTRED_USERS.read().await.keys().count()
}

pub async fn update_user_to_reminder(user: &User, date: NaiveDateTime) {
    LAST_REMINDED_TIME.write().await.insert(user.clone(), date);
    info!(
        "Updated last updated time for user {} as {}",
        user.name, date
    );

    // Update database
    if let Err(e) = database::update_user_last_reminded(user, date).await {
        error!("Failed to update user {} in database: {}", user.name, e);
    }
}

pub async fn remove_user_from_reminders(user: &User) {
    REGISTRED_USERS.write().await.remove(user);
    LAST_REMINDED_TIME.write().await.remove(user);
    info!("Removed {} from registred users", user.name);

    // Remove from database
    if let Err(e) = database::remove_user_reminder(user).await {
        error!("Failed to remove user {} from database: {}", user.name, e);
    }
}

pub async fn load_users_from_database(cache_http: impl CacheHttp) {
    match database::load_user_reminders().await {
        Ok((_users_data, _last_reminded_times_data)) => {
            // Load users from the database into the registries
            for (user, frequency) in _users_data {
                let user_id: UserId = UserId::new(user.id);
                if let Ok(user) = user_id.to_user(&cache_http).await {
                    insert_new_user_to_remind(&user, frequency).await;
                } else {
                    error!(
                        "Failed to load user {} from database: {}",
                        user.name, user.id
                    );
                }
            }
        }
        Err(e) => {
            error!("Failed to load users from database: {}", e);
        }
    }
}
