use chrono::NaiveDateTime;
use rusqlite::{Connection, Result as SqliteResult};
use serenity::all::User;
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use crate::registry::ReminderFrequency;

pub static DATABASE: LazyLock<Mutex<Option<Connection>>> = LazyLock::new(|| Mutex::new(None));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserData {
    pub id: u64,
    pub name: String,
}

impl UserData {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}

impl From<&User> for UserData {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.get(),
            name: user.name.clone(),
        }
    }
}

pub async fn init_database() -> SqliteResult<()> {
    let db_path = "database.sqlite";

    if !Path::new(db_path).exists() {
        info!("Database file does not exist, creating new database");
        create_database(db_path).await?;
    } else {
        info!("Database file exists, connecting to existing database");
        let conn = Connection::open(db_path)?;
        *DATABASE.lock().await = Some(conn);
    }

    Ok(())
}

async fn create_database(db_path: &str) -> SqliteResult<()> {
    let conn = Connection::open(db_path)?;

    // Create users table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            user_id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            reminder_frequency TEXT NOT NULL,
            last_reminded TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    // Create custom messages table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS custom_messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            message_type TEXT NOT NULL,
            message TEXT NOT NULL,
            created_at TEXT NOT NULL,
            UNIQUE(message_type)
        )",
        [],
    )?;

    info!("Database created successfully");
    *DATABASE.lock().await = Some(conn);
    Ok(())
}

pub async fn save_user_reminder(
    user: &User,
    frequency: ReminderFrequency,
    last_reminded: NaiveDateTime,
) -> SqliteResult<()> {
    let db_guard = DATABASE.lock().await;
    let conn = db_guard
        .as_ref()
        .ok_or_else(|| rusqlite::Error::InvalidPath("Database not initialized".into()))?;

    let frequency_str = match frequency {
        ReminderFrequency::ThirtyMin => "ThirtyMin",
        ReminderFrequency::OneHour => "OneHour",
        ReminderFrequency::ThreeHours => "ThreeHours",
    };

    conn.execute(
        "INSERT OR REPLACE INTO users (user_id, username, reminder_frequency, last_reminded, created_at) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            user.id.get() as i64,
            user.name,
            frequency_str,
            last_reminded.to_string(),
            chrono::Utc::now().naive_utc().to_string()
        ],
    )?;

    info!("Saved user {} reminder to database", user.name);
    Ok(())
}

pub async fn load_user_reminders() -> SqliteResult<(
    HashMap<UserData, ReminderFrequency>,
    HashMap<UserData, NaiveDateTime>,
)> {
    let db_guard = DATABASE.lock().await;
    let conn = db_guard.as_ref().ok_or_else(|| {
        rusqlite::Error::InvalidPath("Database not initialized".to_string().into())
    })?;

    let mut stmt =
        conn.prepare("SELECT user_id, username, reminder_frequency, last_reminded FROM users")?;

    let user_iter = stmt.query_map([], |row| {
        let user_id: i64 = row.get(0)?;
        let username: String = row.get(1)?;
        let frequency_str: String = row.get(2)?;
        let last_reminded_str: String = row.get(3)?;

        let frequency = match frequency_str.as_str() {
            "ThirtyMin" => ReminderFrequency::ThirtyMin,
            "OneHour" => ReminderFrequency::OneHour,
            "ThreeHours" => ReminderFrequency::ThreeHours,
            _ => {
                warn!(
                    "Unknown frequency '{}' for user {}, defaulting to OneHour",
                    frequency_str, username
                );
                ReminderFrequency::OneHour
            }
        };

        let last_reminded = NaiveDateTime::parse_from_str(&last_reminded_str, "%Y-%m-%d %H:%M:%S")
            .unwrap_or_else(|_| {
                warn!(
                    "Invalid date format for user {}, using current time",
                    username
                );
                chrono::Utc::now().naive_utc()
            });

        Ok((user_id, username, frequency, last_reminded))
    })?;

    let mut registered_users = HashMap::new();
    let mut last_reminded_times = HashMap::new();

    for result in user_iter {
        match result {
            Ok((user_id, username, frequency, last_reminded)) => {
                let user_data = UserData::new(user_id as u64, username.clone());
                registered_users.insert(user_data.clone(), frequency);
                last_reminded_times.insert(user_data, last_reminded);
                info!("Loaded user {} with frequency {:?}", username, frequency);
            }
            Err(e) => {
                error!("Error loading user from database: {}", e);
            }
        }
    }

    info!("Loaded {} users from database", registered_users.len());
    Ok((registered_users, last_reminded_times))
}

pub async fn remove_user_reminder(user: &User) -> SqliteResult<()> {
    let db_guard = DATABASE.lock().await;
    let conn = db_guard.as_ref().ok_or_else(|| {
        rusqlite::Error::InvalidPath("Database not initialized".to_string().into())
    })?;

    conn.execute(
        "DELETE FROM users WHERE user_id = ?1",
        rusqlite::params![user.id.get() as i64],
    )?;

    info!("Removed user {} reminder from database", user.name);
    Ok(())
}

pub async fn update_user_last_reminded(
    user: &User,
    last_reminded: NaiveDateTime,
) -> SqliteResult<()> {
    let db_guard = DATABASE.lock().await;
    let conn = db_guard.as_ref().ok_or_else(|| {
        rusqlite::Error::InvalidPath("Database not initialized".to_string().into())
    })?;

    conn.execute(
        "UPDATE users SET last_reminded = ?1 WHERE user_id = ?2",
        rusqlite::params![last_reminded.to_string(), user.id.get() as i64],
    )?;

    info!(
        "Updated last reminded time for user {} in database",
        user.name
    );
    Ok(())
}

pub async fn add_custom_message(
    user_id: u64,
    message_type: &str,
    message: &str,
) -> SqliteResult<()> {
    let db_guard = DATABASE.lock().await;
    let conn = db_guard.as_ref().ok_or_else(|| {
        rusqlite::Error::InvalidPath("Database not initialized".to_string().into())
    })?;

    conn.execute(
        "INSERT OR REPLACE INTO custom_messages (user_id, message_type, message, created_at) 
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![
            user_id as i64,
            message_type,
            message,
            chrono::Utc::now().naive_utc().to_string()
        ],
    )?;

    info!(
        "Added custom message for type '{}' by user {}",
        message_type, user_id
    );
    Ok(())
}

pub async fn get_custom_message(message_type: &str) -> SqliteResult<Option<String>> {
    let db_guard = DATABASE.lock().await;
    let conn = db_guard.as_ref().ok_or_else(|| {
        rusqlite::Error::InvalidPath("Database not initialized".to_string().into())
    })?;

    let mut stmt = conn.prepare("SELECT message FROM custom_messages WHERE message_type = ?1")?;

    let mut rows = stmt.query(rusqlite::params![message_type])?;

    if let Some(row) = rows.next()? {
        let message: String = row.get(0)?;
        Ok(Some(message))
    } else {
        Ok(None)
    }
}

pub async fn remove_custom_message(message_type: &str) -> SqliteResult<()> {
    let db_guard = DATABASE.lock().await;
    let conn = db_guard.as_ref().ok_or_else(|| {
        rusqlite::Error::InvalidPath("Database not initialized".to_string().into())
    })?;

    conn.execute(
        "DELETE FROM custom_messages WHERE message_type = ?1",
        rusqlite::params![message_type],
    )?;

    info!("Removed custom message for type '{}'", message_type);
    Ok(())
}

pub async fn list_custom_messages() -> SqliteResult<Vec<(String, String, u64)>> {
    let db_guard = DATABASE.lock().await;
    let conn = db_guard.as_ref().ok_or_else(|| {
        rusqlite::Error::InvalidPath("Database not initialized".to_string().into())
    })?;

    let mut stmt = conn.prepare(
        "SELECT message_type, message, user_id FROM custom_messages ORDER BY message_type",
    )?;

    let rows = stmt.query_map([], |row| {
        let message_type: String = row.get(0)?;
        let message: String = row.get(1)?;
        let user_id: i64 = row.get(2)?;
        Ok((message_type, message, user_id as u64))
    })?;

    let mut messages = Vec::new();
    for result in rows {
        messages.push(result?);
    }

    Ok(messages)
}
