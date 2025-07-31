use chrono::TimeDelta;
use serenity::all::{CacheHttp, CreateMessage, User};

use crate::registry::{
    LAST_REMINDED_TIME, REGISTRED_USERS, ReminderFrequency, update_user_to_reminder,
};

async fn dm_user_reminder(cache_http: &impl CacheHttp, user: &User, freq: ReminderFrequency) {
    let content = match freq {
        ReminderFrequency::ThirtyMin => "💧 C'est l'heure de boire un peu d'eau ! 💧",
        ReminderFrequency::OneHour => "💧 C'est l'heure de boire un verre d'eau ! 💧",
        ReminderFrequency::ThreeHours => "💧 C'est l'heure de boire une grande quantité d'eau ! 💧",
    };

    let _res = user
        .dm(cache_http, CreateMessage::new().content(content))
        .await;
}

pub async fn walk_reminders(cache_http: impl CacheHttp) {
    let now = chrono::Utc::now().naive_utc();

    let last_reminded_time_guard = LAST_REMINDED_TIME.read().await;

    for (user, freq) in REGISTRED_USERS.read().await.iter() {
        let Some(last_reminded) = last_reminded_time_guard.get(user) else {
            continue;
        };

        let delta = TimeDelta::from(*freq);
        let Some(limit) = last_reminded.checked_add_signed(delta) else {
            eprintln!(
                "Unable to add {} with {} to check for reminders of user {}",
                last_reminded, delta, user
            );
            continue;
        };

        if limit < now {
            dm_user_reminder(&cache_http, user, *freq).await;
        }
    }

    drop(last_reminded_time_guard);

    for user in REGISTRED_USERS.read().await.keys() {
        update_user_to_reminder(user, now).await;
    }
}
