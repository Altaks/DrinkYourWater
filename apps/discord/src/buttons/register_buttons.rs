use std::fmt::Display;

use serenity::all::{ButtonStyle, CreateButton};

use crate::registry::ReminderFrequency;

pub const BUTTON_30_MIN_ID: &str = "button30min";
pub const BUTTON_1_HOUR_ID: &str = "button1h";
pub const BUTTON_3_HOURS_ID: &str = "button3h";

pub fn get_30min_button() -> CreateButton {
    CreateButton::new(BUTTON_30_MIN_ID)
        .label("30min")
        .emoji('💧')
        .style(ButtonStyle::Secondary)
}

pub fn get_1h_button() -> CreateButton {
    CreateButton::new(BUTTON_1_HOUR_ID)
        .label("1h")
        .emoji('💦')
        .style(ButtonStyle::Primary)
}

pub fn get_3h_button() -> CreateButton {
    CreateButton::new(BUTTON_3_HOURS_ID)
        .label("3h")
        .emoji('🌊')
        .style(ButtonStyle::Primary)
}

pub fn resolve_user_choice(choice: &String) -> Result<ReminderFrequency, serenity::Error> {
    let frequency = match choice.as_ref() {
        BUTTON_30_MIN_ID => ReminderFrequency::ThirtyMin,
        BUTTON_1_HOUR_ID => ReminderFrequency::OneHour,
        BUTTON_3_HOURS_ID => ReminderFrequency::ThreeHours,
        _ => return Err(serenity::Error::Other("This value shouldn't exist >:(")),
    };

    Ok(frequency)
}

impl Display for ReminderFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReminderFrequency::ThirtyMin => write!(f, "30 min"),
            ReminderFrequency::OneHour => write!(f, "1 heure"),
            ReminderFrequency::ThreeHours => write!(f, "3 heures"),
        }
    }
}
