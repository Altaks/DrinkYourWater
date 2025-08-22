use crate::data::messages::*;
use crate::database::get_custom_message;
use chrono::TimeDelta;
use rand::random_range;
use serenity::all::{CacheHttp, CreateMessage, User};
use tracing::{error, info, warn};

use crate::registry::{
    LAST_REMINDED_TIME, REGISTRED_USERS, ReminderFrequency, update_user_to_reminder,
};

async fn dm_user_reminder(cache_http: &impl CacheHttp, user: &User, freq: ReminderFrequency) {
    // Try to get custom message first
    let message_type = match freq {
        ReminderFrequency::ThirtyMin => "thirty_min",
        ReminderFrequency::OneHour => "one_hour",
        ReminderFrequency::ThreeHours => "three_hours",
    };

    let content = if let Ok(Some(custom_msg)) = get_custom_message(message_type).await {
        // Use custom message
        custom_msg
    } else {
        // Fall back to default messages
        let default_content: &'static str = match freq {
            ReminderFrequency::ThirtyMin => REMINDER_MESSAGE_THIRTY_MIN
                .get(random_range(0..(REMINDER_MESSAGE_THIRTY_MIN.len())))
                .unwrap_or(&ERROR_MESSAGE),
            ReminderFrequency::OneHour => REMINDER_MESSAGE_ONE_HOUR
                .get(random_range(0..(REMINDER_MESSAGE_ONE_HOUR.len())))
                .unwrap_or(&ERROR_MESSAGE),
            ReminderFrequency::ThreeHours => REMINDER_MESSAGE_THREE_HOURS
                .get(random_range(0..(REMINDER_MESSAGE_THREE_HOURS.len())))
                .unwrap_or(&ERROR_MESSAGE),
        };
        default_content.to_string()
    };

    info!("DM'ing user {} for its reminder", user.name);
    let _res = user
        .dm(cache_http, CreateMessage::new().content(content))
        .await;
    info!("Finished DM'ing user {} for its reminder", user.name);
}

pub async fn walk_reminders(cache_http: impl CacheHttp) {
    info!("Starting to walk through enabled reminders");

    let now = chrono::Utc::now().naive_utc();
    info!("Time is currently : {}", now);

    let last_reminded_time_guard = LAST_REMINDED_TIME.read().await;

    for (user, last_remind) in last_reminded_time_guard.iter() {
        info!(
            "User {} last remind is at {} with a frequency of {:?}",
            user.name,
            last_remind,
            REGISTRED_USERS.read().await.get(user)
        );
    }

    let mut usernames_reminded = Vec::<String>::new();

    for (user, freq) in REGISTRED_USERS.read().await.iter() {
        info!("Checking for user {}", user.name);

        let Some(last_reminded) = last_reminded_time_guard.get(user) else {
            warn!(
                "User {} has no last reminded time :(, skipping...",
                user.name
            );
            continue;
        };

        let delta = TimeDelta::from(*freq);
        info!("Created time delta from user preferred reminder frequency");

        let Some(limit) = last_reminded.checked_add_signed(delta) else {
            error!(
                "Unable to add {} with {} to check for reminders of user {}",
                last_reminded, delta, user
            );
            continue;
        };
        info!("Limit time has been computed");

        if limit < now {
            dm_user_reminder(&cache_http, user, *freq).await;
            usernames_reminded.push(user.name.clone());
        } else {
            info!(
                "User {} time limit for reminder is not passed yet : {}",
                user.name, limit
            );
        }
    }

    info!("Dropping last reminded time guard");
    drop(last_reminded_time_guard);

    info!("Updating every single reminder time after the scan");
    for user in REGISTRED_USERS.read().await.keys() {
        if usernames_reminded.contains(&user.name) {
            update_user_to_reminder(user, now).await;
        }
    }

    info!("Finished scanning &/ processing all the reminders");
}
