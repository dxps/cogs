use crate::{
    infra::{new_app_error_from_sqlx, new_id},
    server::AuthUserAccount,
};
use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::{Id, UserAccount, UserEntry, UserPasswordSalt},
};
use sqlx::{PgPool, Row, postgres::PgRow};
use std::sync::Arc;

#[derive(Debug)]
pub struct UserAccountsRepo {
    dbcp: Arc<PgPool>,
}

impl UserAccountsRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub fn from(pool: &PgPool) -> Self {
        Self {
            dbcp: Arc::new(pool.clone()),
        }
    }

    pub async fn get_by_username(&self, username: &String) -> AppResult<UserEntry> {
        //
        let row = sqlx::query(
            "SELECT id, name, email, password, salt, bio, is_anonymous FROM user_accounts 
             WHERE username = $1",
        )
        .bind(username)
        .fetch_one(self.dbcp.as_ref())
        .await
        .map_err(|err| new_app_error_from_sqlx(err, Some("failed to get user by email".to_string())))?;

        let mut user_account = UserAccount {
            id: Id(row.get("id")),
            name: row.get("name"),
            email: row.get("email"),
            username: username.clone(),
            bio: row.get("bio"),
            is_anonymous: row.get("is_anonymous"),
            permissions: Vec::new(),
        };

        let permissions = sqlx::query("SELECT permission FROM user_permissions WHERE user_id = $1")
            .bind(user_account.id.0)
            .fetch_all(self.dbcp.as_ref())
            .await
            .map_err(|err| new_app_error_from_sqlx(err, Some("failed to get user permissions".to_string())))?;

        user_account.permissions = permissions.iter().map(|r| r.get("permission")).collect();

        Ok(UserEntry {
            user: user_account,
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }

    pub async fn get_by_id(id: &Id, pool: &PgPool) -> Option<AuthUserAccount> {
        //
        let row = sqlx::query("SELECT id, email, username, bio, is_anyonymous FROM user_accounts WHERE id = $1")
            .bind(id.to_string())
            .fetch_one(pool)
            .await
            .ok()?;

        let mut user_account = UserAccount {
            id: Id(row.get("id")),
            name: row.get("name"),
            email: row.get("email"),
            username: row.get("username"),
            bio: row.get("bio"),
            is_anonymous: row.get("is_anonymous"),
            permissions: Vec::new(),
        };

        let mut permissions = sqlx::query("SELECT permission FROM user_permissions WHERE user_id = $1;")
            .map(|r: PgRow| r.get("permission"))
            .fetch_all(pool)
            .await
            .ok()?;

        user_account.permissions.append(&mut permissions);
        Some(user_account.into())
    }

    pub async fn get_permissions(&self, account: &mut UserAccount) -> AppResult<()> {
        let mut permissions = sqlx::query("SELECT permission FROM user_permissions WHERE user_id = $1;")
            .bind(&account.id.0)
            .map(|r: PgRow| r.get("permission"))
            .fetch_all(self.dbcp.as_ref())
            .await
            .map_err(|err| {
                log::error!(
                    "Could not load permissions for user account w/ id: {}. Error: {err}",
                    account.id
                );
                AppError::from(err.to_string())
            })?;
        account.permissions.append(&mut permissions);
        Ok(())
    }

    pub async fn save_with_permissions(
        &self,
        name: &String,
        email: &String,
        username: &String,
        pwd: &String,
        salt: &String,
        permissions: Vec<String>,
    ) -> AppResult<Id> {
        //
        let id = new_id();
        let res = sqlx::query(
            "INSERT INTO user_accounts (id, name, email, username, password, salt) 
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(&id.0)
        .bind(name)
        .bind(email)
        .bind(username)
        .bind(pwd)
        .bind(salt)
        .execute(self.dbcp.as_ref())
        .await
        .map_err(|err| new_app_error_from_sqlx(err, None));

        if res.is_ok() {
            for permission in permissions.iter() {
                let res = sqlx::query("INSERT INTO user_permissions (user_id, permission) VALUES ($1, $2)")
                    .bind(&id.0)
                    .bind(&permission)
                    .execute(self.dbcp.as_ref())
                    .await
                    .map_err(|err| new_app_error_from_sqlx(err, None));
                if res.is_err() {
                    return AppResult::Err(res.err().unwrap());
                }
            }
        } else {
            return AppResult::Err(res.err().unwrap());
        }
        AppResult::Ok(id)
    }

    pub async fn get_password_by_id(&self, user_id: &Id) -> AppResult<UserPasswordSalt> {
        //
        let row = sqlx::query("SELECT password, salt FROM user_accounts WHERE id = $1")
            .bind(user_id.0)
            .fetch_one(self.dbcp.as_ref())
            .await
            .map_err(|err| new_app_error_from_sqlx(err, Some("failed to get password by user id".to_string())))?;

        Ok(UserPasswordSalt {
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }

    pub async fn update_password(&self, user_id: &Id, pwd: String) -> AppResult<()> {
        //
        match sqlx::query("UPDATE user_accounts SET password = $1 WHERE id = $2")
            .bind(pwd)
            .bind(user_id.0)
            .execute(self.dbcp.as_ref())
            .await
            .map_err(|err| AppError::from(err.to_string()))
        {
            Ok(_) => Ok(()),
            Err(err) => Err(AppError::from(err)),
        }
    }
}
