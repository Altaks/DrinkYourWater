use chrono::TimeDelta;
use serenity::all::{CacheHttp, CreateMessage, User};

use crate::registry::{LAST_REMINDED_TIME, REGISTRED_USERS, update_user_to_reminder};

async fn dm_user_reminder(cache_http: &impl CacheHttp, user: &User) {
    println!("DM'ing {} for a reminder", user.name);
    let _res = user
        .dm(cache_http, CreateMessage::new().content("Reminder"))
        .await;
}

pub async fn walk_reminders(cache_http: impl CacheHttp) {
    println!("Checking for reminders");
    let now = chrono::Utc::now().naive_utc();
    println!("Figured out time");

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
            dm_user_reminder(&cache_http, user).await;
        }
    }

    for user in REGISTRED_USERS.read().await.keys() {
        update_user_to_reminder(user, now).await;
    }

    println!("Finished checking for reminders");
}
