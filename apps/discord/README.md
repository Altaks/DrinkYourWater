# DrinkYourWater Discord Bot

A Discord bot that reminds users to drink water at regular intervals.

## Features

- **Water Reminder System**: Users can register to receive reminders to drink water
- **Multiple Reminder Frequencies**: Choose from 30 minutes, 1 hour, or 3 hours
- **Persistent Storage**: User reminders are saved to a SQLite database (`database.sqlite`)
- **Automatic Loading**: On startup, the bot loads existing user reminders from the database
- **Direct Message Reminders**: Users receive personalized reminders via Discord DMs

## Commands

### `/register`
Register yourself (or another user) to receive water drinking reminders.

**Options:**
- `target` (optional): The user to register. If not specified, registers the command user.

**Usage:**
1. Run `/register` or `/register @user`
2. Choose your preferred reminder frequency (30min, 1h, or 3h)
3. You'll start receiving reminders at the selected interval

### `/unregister`
Unregister yourself from water drinking reminders.

**Usage:**
- Run `/unregister` to stop receiving reminders

## Database

The bot uses SQLite to persistently store user reminder data:

- **File**: `database.sqlite` (created automatically in the bot's directory)
- **Table**: `users`
  - `user_id`: Discord user ID (primary key)
  - `username`: Discord username
  - `reminder_frequency`: Reminder frequency (ThirtyMin, OneHour, ThreeHours)
  - `last_reminded`: Timestamp of last reminder
  - `created_at`: Timestamp when user was registered

## Setup

1. **Environment Variables**: Create a `.env` file with:
   ```
   DISCORD_BOT_TOKEN=your_discord_bot_token
   DISCORD_GUILD_ID=your_guild_id
   ```

2. **Build and Run**:
   ```bash
   cargo build
   cargo run
   ```

3. **Database**: The SQLite database will be created automatically on first run.

## Technical Details

- **Framework**: Serenity (Discord API wrapper)
- **Database**: SQLite with rusqlite
- **Async Runtime**: Tokio
- **Logging**: Tracing
- **Error Handling**: Anyhow

## Reminder Messages

The bot sends different messages based on the reminder frequency:
- **30 minutes**: "💧 C'est l'heure de boire un peu d'eau ! 💧"
- **1 hour**: "💧 C'est l'heure de boire un verre d'eau ! 💧"
- **3 hours**: "💧 C'est l'heure de boire une grande quantité d'eau ! 💧"

## Notes

- Reminders are sent every minute (the bot checks if it's time to remind each user)
- Users receive reminders via Discord Direct Messages
- The bot automatically handles timezone conversions using UTC
- Database operations are thread-safe using Mutex for SQLite connections 
