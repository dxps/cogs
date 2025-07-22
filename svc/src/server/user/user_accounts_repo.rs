use sqlx::{FromRow, PgPool, Row, postgres::PgRow};
use std::sync::Arc;

use crate::{
    app_err_uc::{AppError, AppResult, AppUseCase},
    domain::model::{Id, UserAccount, UserEntry, UserPasswordSalt},
    server::generate_id,
};

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

    pub async fn get_by_username(
        &self,
        username: &String,
        usecase: AppUseCase,
    ) -> AppResult<UserEntry> {
        //
        sqlx::query_as::<_, UserEntry>(
            "SELECT id, email, username, password, name, salt, bio, is_anonymous FROM user_accounts 
             WHERE username = $1",
        )
        .bind(username)
        .fetch_one(self.dbcp.as_ref())
        .await
        .map_err(|err| AppError::from((err, usecase)))
    }

    pub async fn get_by_email(&self, email: &String, usecase: AppUseCase) -> AppResult<UserEntry> {
        //
        sqlx::query_as::<_, UserEntry>(
            "SELECT id, email, username, password, name, salt, bio, is_anonymous FROM user_accounts 
             WHERE email = $1",
        )
        .bind(email)
        .fetch_one(self.dbcp.as_ref())
        .await
        .map_err(|err| AppError::from((err, usecase)))
    }

    pub async fn get_by_id(&self, id: &Id) -> AppResult<Option<UserAccount>> {
        //
        let mut account = sqlx::query_as::<_, UserAccount>(
            "SELECT id, name, email, username, bio, is_anonymous FROM user_accounts WHERE id = $1",
        )
        .bind(id.as_str())
        .fetch_one(self.dbcp.as_ref())
        .await
        .map_err(AppError::from)?;

        self.get_permissions(&mut account).await?;

        Ok(Some(account))
    }

    pub async fn get_permissions(&self, account: &mut UserAccount) -> AppResult<()> {
        let mut permissions =
            sqlx::query("SELECT permission FROM user_permissions WHERE user_id = $1;")
                .bind(account.id.as_str())
                .map(|r: PgRow| r.get("permission"))
                .fetch_all(self.dbcp.as_ref())
                .await
                .map_err(|err| {
                    log::error!(
                        "Could not load permissions for user account w/ id: {}. Error: {err}",
                        account.id
                    );
                    AppError::from(err)
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
        let id = generate_id();
        let res = sqlx::query(
            "INSERT INTO user_accounts (id, name, email, username, password, salt) 
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(id.as_str())
        .bind(name)
        .bind(email)
        .bind(username)
        .bind(pwd)
        .bind(salt)
        .execute(self.dbcp.as_ref())
        .await
        .map_err(|err| {
            AppError::from((
                err,
                AppUseCase::UserRegistration,
                "Self registration of admin user account".to_string(),
            ))
        });

        if res.is_ok() {
            for permission in permissions.iter() {
                let res = sqlx::query(
                    "INSERT INTO user_permissions (user_id, permission) VALUES ($1, $2)",
                )
                .bind(&id.as_str())
                .bind(&permission)
                .execute(self.dbcp.as_ref())
                .await
                .map_err(|err| {
                    AppError::from((
                        err,
                        AppUseCase::UserRegistration,
                        "Self registration of admin user permissions".to_string(),
                    ))
                });
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
        sqlx::query_as::<_, UserPasswordSalt>(
            "SELECT password, salt FROM user_accounts WHERE id = $1",
        )
        .bind(user_id.as_str())
        .fetch_one(self.dbcp.as_ref())
        .await
        .map_err(|err| AppError::from(err))
    }

    pub async fn update_password(&self, user_id: &Id, pwd: String) -> AppResult<()> {
        //
        match sqlx::query("UPDATE user_accounts SET password = $1 WHERE id = $2")
            .bind(pwd)
            .bind(user_id.as_str())
            .execute(self.dbcp.as_ref())
            .await
            .map_err(|err| AppError::from(err))
        {
            Ok(_) => Ok(()),
            Err(err) => Err(AppError::from(err)),
        }
    }
}

// ///////////////////////////////////////////
// Implementations of sqlx's FromRow trait. //
// ///////////////////////////////////////////

impl FromRow<'_, PgRow> for UserEntry {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user: UserAccount {
                id: Id::new_from(row.try_get("id").unwrap_or_default()),
                email: row.get("email"),
                username: row.get("username"),
                name: row.get("name"),
                bio: row.get("bio"),
                is_anonymous: row.get("is_anonymous"),
                permissions: Vec::new(),
            },
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }
}

impl FromRow<'_, PgRow> for UserPasswordSalt {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }
}

impl FromRow<'_, PgRow> for UserAccount {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: Id::new_from(row.get("id")),
            email: row.get("email"),
            username: row.get("username"),
            name: row.get("name"),
            bio: row.get("bio"),
            is_anonymous: row.get("is_anonymous"),
            permissions: Vec::new(),
        })
    }
}
