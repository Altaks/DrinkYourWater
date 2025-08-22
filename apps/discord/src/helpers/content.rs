use crate::data::messages::*;
use crate::registry::ReminderFrequency;
use rand::random_range;

#[inline(always)]
pub fn display_message_type(msg_type: &str) -> &'static str {
    match msg_type {
        "thirty_min" => "30 minutes",
        "one_hour" => "1 heure",
        "three_hours" => "3 heures",
        _ => "unknown",
    }
}

#[inline(always)]
pub fn get_default_message(freq: ReminderFrequency) -> &'static str {
    let messages: &[&str] = match freq {
        ReminderFrequency::ThirtyMin => &REMINDER_MESSAGE_THIRTY_MIN,
        ReminderFrequency::OneHour => &REMINDER_MESSAGE_ONE_HOUR,
        ReminderFrequency::ThreeHours => &REMINDER_MESSAGE_THREE_HOURS,
    };

    if messages.is_empty() {
        &ERROR_MESSAGE
    } else {
        messages
            .get(random_range(0..messages.len()))
            .unwrap_or(&ERROR_MESSAGE)
    }
}
