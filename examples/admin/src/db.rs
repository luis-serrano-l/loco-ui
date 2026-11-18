//! The database: four tables, the seed, and every query the handlers run. All SQL is here
//! and static; values are bound, never formatted in.

use std::str::FromStr;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

/// The pool the router holds.
pub type Db = sqlx::SqlitePool;

const SCHEMA: &[&str] = &[
    "CREATE TABLE IF NOT EXISTS roles (
        id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE, description TEXT NOT NULL)",
    "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT NOT NULL UNIQUE,
        password_hash TEXT NOT NULL, role_id INTEGER NOT NULL REFERENCES roles(id),
        active INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL DEFAULT (datetime('now')))",
    "CREATE TABLE IF NOT EXISTS sessions (
        token TEXT PRIMARY KEY, user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE)",
    "CREATE TABLE IF NOT EXISTS audit (
        id INTEGER PRIMARY KEY, at TEXT NOT NULL DEFAULT (datetime('now')),
        actor TEXT NOT NULL, action TEXT NOT NULL, target TEXT NOT NULL)",
];

/// The account the seed creates.
pub const SEED_EMAIL: &str = "ada@example.com";
/// Its password.
pub const SEED_PASSWORD: &str = "analytical-engine";

const ROLES: [(&str, &str); 4] = [
    (
        "Owner",
        "Everything, including billing and deleting the workspace.",
    ),
    ("Admin", "Manages users and roles."),
    ("Member", "Works in the product."),
    ("Viewer", "Reads, changes nothing."),
];

const PEOPLE: [(&str, &str, i64, bool); 9] = [
    ("Grace Hopper", "grace@example.com", 2, true),
    ("Alan Turing", "alan@example.com", 2, true),
    ("Katherine Johnson", "katherine@example.com", 3, true),
    ("Edsger Dijkstra", "edsger@example.com", 3, false),
    ("Barbara Liskov", "barbara@example.com", 3, true),
    ("Donald Knuth", "donald@example.com", 3, true),
    ("Margaret Hamilton", "margaret@example.com", 3, true),
    ("Tim Berners-Lee", "tim@example.com", 4, false),
    ("Radia Perlman", "radia@example.com", 4, true),
];

/// Open (and create) the database at `url`, with its tables and, when it has no user yet,
/// the seed. One connection: an in-memory database lives in its connection, and SQLite
/// writes one at a time anyway.
pub async fn open(url: &str) -> Result<Db, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(url)?
        .create_if_missing(true)
        .foreign_keys(true);
    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    for sql in SCHEMA {
        sqlx::query(*sql).execute(&db).await?;
    }
    let users: i64 = sqlx::query_scalar("SELECT count(*) FROM users")
        .fetch_one(&db)
        .await?;
    if users == 0 {
        seed(&db).await?;
    }
    Ok(db)
}

async fn seed(db: &Db) -> Result<(), sqlx::Error> {
    for (name, description) in ROLES {
        create_role(db, name, description).await?;
    }
    // One hash for every seeded account: hashing is slow on purpose.
    let hash = crate::auth::hash(SEED_PASSWORD);
    create_user(db, "Ada Lovelace", SEED_EMAIL, &hash, 1, true).await?;
    for (name, email, role, active) in PEOPLE {
        create_user(db, name, email, &hash, role, active).await?;
    }
    log(db, "System", "seeded", "10 users, 4 roles").await?;
    // Fixed dates, nine days apart, so the seed reads like a history and the test's
    // screenshots are the same on every run.
    sqlx::query(
        "UPDATE users SET created_at = datetime('2026-01-05 09:00:00', '+' || (id * 9) || ' days')",
    )
    .execute(db)
    .await?;
    sqlx::query("UPDATE audit SET at = '2026-01-05 09:00:00'")
        .execute(db)
        .await?;
    Ok(())
}

/// A user as the pages show one.
#[derive(Clone, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub role_id: i64,
    pub role: String,
    pub active: bool,
    pub created: String,
}

type UserRow = (i64, String, String, i64, String, bool, String);

fn user(r: UserRow) -> User {
    User {
        id: r.0,
        name: r.1,
        email: r.2,
        role_id: r.3,
        role: r.4,
        active: r.5,
        created: r.6,
    }
}

/// Users whose name or email holds `q` (as text: `%` and `_` are themselves), with `status`
/// ("active", "suspended", anything else for both) and the role `role` (0 for any), by name.
pub async fn users(db: &Db, q: &str, status: &str, role: i64) -> Result<Vec<User>, sqlx::Error> {
    let rows: Vec<UserRow> = sqlx::query_as(
        "SELECT u.id, u.name, u.email, u.role_id, r.name, u.active, u.created_at
         FROM users u JOIN roles r ON r.id = u.role_id
         WHERE (instr(lower(u.name), lower(?1)) > 0 OR instr(lower(u.email), lower(?1)) > 0)
           AND (?2 NOT IN ('active', 'suspended') OR u.active = (?2 = 'active'))
           AND (?3 = 0 OR u.role_id = ?3)
         ORDER BY u.name",
    )
    .bind(q)
    .bind(status)
    .bind(role)
    .fetch_all(db)
    .await?;
    Ok(rows.into_iter().map(user).collect())
}

/// Whether there is a role with this id.
pub async fn role_exists(db: &Db, id: i64) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM roles WHERE id = ?1)")
        .bind(id)
        .fetch_one(db)
        .await
}

/// How many users there are: all, and the active ones.
pub async fn user_counts(db: &Db) -> Result<(i64, i64), sqlx::Error> {
    sqlx::query_as("SELECT count(*), coalesce(sum(active), 0) FROM users")
        .fetch_one(db)
        .await
}

/// The user `id`.
pub async fn find_user(db: &Db, id: i64) -> Result<Option<User>, sqlx::Error> {
    let row: Option<UserRow> = sqlx::query_as(
        "SELECT u.id, u.name, u.email, u.role_id, r.name, u.active, u.created_at
         FROM users u JOIN roles r ON r.id = u.role_id WHERE u.id = ?1",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;
    Ok(row.map(user))
}

/// Whether another user than `except` has `email`.
pub async fn email_taken(db: &Db, email: &str, except: i64) -> Result<bool, sqlx::Error> {
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM users WHERE email = ?1 AND id != ?2")
        .bind(email)
        .bind(except)
        .fetch_one(db)
        .await?;
    Ok(n > 0)
}

/// Add a user; its id.
pub async fn create_user(
    db: &Db,
    name: &str,
    email: &str,
    hash: &str,
    role: i64,
    active: bool,
) -> Result<i64, sqlx::Error> {
    let done = sqlx::query("INSERT INTO users (name, email, password_hash, role_id, active) VALUES (?1, ?2, ?3, ?4, ?5)")
        .bind(name)
        .bind(email)
        .bind(hash)
        .bind(role)
        .bind(active)
        .execute(db)
        .await?;
    Ok(done.last_insert_rowid())
}

/// Change a user's name, email, role and status. A suspended user's sessions end.
pub async fn update_user(
    db: &Db,
    id: i64,
    name: &str,
    email: &str,
    role: i64,
    active: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET name = ?2, email = ?3, role_id = ?4, active = ?5 WHERE id = ?1")
        .bind(id)
        .bind(name)
        .bind(email)
        .bind(role)
        .bind(active)
        .execute(db)
        .await?;
    if !active {
        sqlx::query("DELETE FROM sessions WHERE user_id = ?1")
            .bind(id)
            .execute(db)
            .await?;
    }
    Ok(())
}

/// Remove a user and their sessions.
pub async fn delete_user(db: &Db, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = ?1")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

/// Change a user's name alone (the settings page).
pub async fn rename_user(db: &Db, id: i64, name: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET name = ?2 WHERE id = ?1")
        .bind(id)
        .bind(name)
        .execute(db)
        .await?;
    Ok(())
}

/// The id, password hash and status of the user with `email`.
pub async fn credentials(db: &Db, email: &str) -> Result<Option<(i64, String, bool)>, sqlx::Error> {
    sqlx::query_as("SELECT id, password_hash, active FROM users WHERE email = ?1")
        .bind(email)
        .fetch_optional(db)
        .await
}

/// The password hash of the user `id`.
pub async fn password_hash(db: &Db, id: i64) -> Result<String, sqlx::Error> {
    sqlx::query_scalar("SELECT password_hash FROM users WHERE id = ?1")
        .bind(id)
        .fetch_one(db)
        .await
}

/// Store a new password hash and end the user's other sessions (all but `keep`).
pub async fn set_password(db: &Db, id: i64, hash: &str, keep: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET password_hash = ?2 WHERE id = ?1")
        .bind(id)
        .bind(hash)
        .execute(db)
        .await?;
    sqlx::query("DELETE FROM sessions WHERE user_id = ?1 AND token != ?2")
        .bind(id)
        .bind(keep)
        .execute(db)
        .await?;
    Ok(())
}

/// A role with the number of users who have it.
#[derive(Clone, Debug)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub users: i64,
}

/// Every role, in the order they were made.
pub async fn roles(db: &Db) -> Result<Vec<Role>, sqlx::Error> {
    let rows: Vec<(i64, String, String, i64)> = sqlx::query_as(
        "SELECT r.id, r.name, r.description, (SELECT count(*) FROM users u WHERE u.role_id = r.id)
         FROM roles r ORDER BY r.id",
    )
    .fetch_all(db)
    .await?;
    Ok(rows
        .into_iter()
        .map(|r| Role {
            id: r.0,
            name: r.1,
            description: r.2,
            users: r.3,
        })
        .collect())
}

/// Add a role; `false` when the name is taken.
pub async fn create_role(db: &Db, name: &str, description: &str) -> Result<bool, sqlx::Error> {
    let done = sqlx::query("INSERT OR IGNORE INTO roles (name, description) VALUES (?1, ?2)")
        .bind(name)
        .bind(description)
        .execute(db)
        .await?;
    Ok(done.rows_affected() == 1)
}

/// Remove a role nobody has; `false` when someone still has it.
pub async fn delete_role(db: &Db, id: i64) -> Result<bool, sqlx::Error> {
    let done = sqlx::query(
        "DELETE FROM roles WHERE id = ?1 AND NOT EXISTS (SELECT 1 FROM users WHERE role_id = ?1)",
    )
    .bind(id)
    .execute(db)
    .await?;
    Ok(done.rows_affected() == 1)
}

/// A line of the audit log.
#[derive(Clone, Debug)]
pub struct Entry {
    pub at: String,
    pub actor: String,
    pub action: String,
    pub target: String,
}

/// Write a line: who did what to what.
pub async fn log(db: &Db, actor: &str, action: &str, target: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO audit (actor, action, target) VALUES (?1, ?2, ?3)")
        .bind(actor)
        .bind(action)
        .bind(target)
        .execute(db)
        .await?;
    Ok(())
}

/// The newest 200 lines that hold `q` and, when `action` is not empty, have that action.
pub async fn audit(db: &Db, q: &str, action: &str) -> Result<Vec<Entry>, sqlx::Error> {
    let rows: Vec<(String, String, String, String)> = sqlx::query_as(
        "SELECT at, actor, action, target FROM audit
         WHERE (instr(lower(actor), lower(?1)) > 0 OR instr(lower(target), lower(?1)) > 0)
           AND (?2 = '' OR action = ?2)
         ORDER BY id DESC LIMIT 200",
    )
    .bind(q)
    .bind(action)
    .fetch_all(db)
    .await?;
    Ok(rows
        .into_iter()
        .map(|r| Entry {
            at: r.0,
            actor: r.1,
            action: r.2,
            target: r.3,
        })
        .collect())
}

/// Start a session for `user`; the token for the cookie (16 random bytes from SQLite).
pub async fn start_session(db: &Db, user: i64) -> Result<String, sqlx::Error> {
    sqlx::query_scalar("INSERT INTO sessions (token, user_id) VALUES (lower(hex(randomblob(16))), ?1) RETURNING token")
        .bind(user)
        .fetch_one(db)
        .await
}

/// End the session `token`.
pub async fn end_session(db: &Db, token: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE token = ?1")
        .bind(token)
        .execute(db)
        .await?;
    Ok(())
}

/// The active user the session `token` belongs to: id and name.
pub async fn session_user(db: &Db, token: &str) -> Result<Option<(i64, String)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT u.id, u.name FROM sessions s JOIN users u ON u.id = s.user_id
         WHERE s.token = ?1 AND u.active = 1",
    )
    .bind(token)
    .fetch_optional(db)
    .await
}
