use std::{collections::HashMap, sync::LazyLock};

use chrono::{NaiveDateTime, TimeDelta};
use serenity::all::{CacheHttp, User};
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
            ReminderFrequency::ThirtyMin => TimeDelta::seconds(10),
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
        Ok((users_data, last_reminded_times_data)) => {
            use crate::registry::{LAST_REMINDED_TIME, REGISTRED_USERS};
            use serenity::all::UserId;
            use tracing::warn;
            let mut loaded = 0;
            for (user_data, freq) in users_data.iter() {
                let user_id = UserId::new(user_data.id);
                match user_id.to_user(&cache_http).await {
                    Ok(user) => {
                        if let Some(date) = last_reminded_times_data.get(user_data) {
                            REGISTRED_USERS.write().await.insert(user.clone(), *freq);
                            LAST_REMINDED_TIME.write().await.insert(user.clone(), *date);
                            loaded += 1;
                        } else {
                            warn!(
                                "Utilisateur {} (id: {}) ignoré : pas de date de dernier rappel.",
                                user_data.name, user_data.id
                            );
                        }
                    }
                    Err(e) => {
                        warn!(
                            "Impossible de charger l'utilisateur {} (id: {}): {}",
                            user_data.name, user_data.id, e
                        );
                    }
                }
            }
            info!(
                "Chargé {}/{} utilisateurs depuis la base de données",
                loaded,
                users_data.len()
            );
        }
        Err(e) => {
            error!(
                "Échec du chargement des utilisateurs depuis la base de données: {}",
                e
            );
        }
    }
}
