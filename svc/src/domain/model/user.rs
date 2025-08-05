use crate::domain::model::Id;
use serde::{Deserialize, Serialize};

/// User account contains most of the details of a user (except password related ones).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAccount {
    pub id: Id,
    pub email: String,
    pub username: String,
    pub name: String,
    pub bio: String,
    pub is_anonymous: bool,
    pub permissions: Vec<String>,
}

impl Default for UserAccount {
    fn default() -> Self {
        Self {
            id: Id::default(),
            is_anonymous: true,
            username: "Guest".into(),
            email: "".into(),
            name: "".into(),
            bio: "".into(),
            permissions: Vec::new(),
        }
    }
}

impl UserAccount {
    pub fn is_admin_read(&self) -> bool {
        self.permissions.contains(&"Admin::Read".into())
    }
}

#[derive(Debug)]
/// It includes all user attributes that are persisted in the database.
pub struct UserEntry {
    pub user: UserAccount,
    pub password: String,
    pub salt: String,
}

impl From<UserEntry> for UserAccount {
    fn from(entry: UserEntry) -> Self {
        entry.user
    }
}

/// It includes just the user's password and salt.
pub struct UserPasswordSalt {
    pub password: String,
    pub salt: String,
}
