use crate::server::UserAccountsRepo;
use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::{Id, UserAccount},
};
use randoid::randoid;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct UserMgmt {
    user_repo: Arc<UserAccountsRepo>,
}

impl UserMgmt {
    //
    pub fn new(user_repo: Arc<UserAccountsRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn authenticate_user(&self, email: String, pwd: String) -> AppResult<UserAccount> {
        //
        let user_entry = self.user_repo.get_by_username(&email).await.map_err(|err| {
            if err == AppError::NotFound {
                AppError::Unauthorized("wrong credentials".into())
            } else {
                err
            }
        })?;
        match Self::check_password(&pwd, &user_entry.password, &user_entry.salt) {
            true => Ok(user_entry.into()),
            false => Err(AppError::Unauthorized("wrong credentials".into())),
        }
    }

    pub async fn register_admin_user(&self, name: String, email: String, username: String, pwd: String) -> AppResult<Id> {
        //
        let (pwd, salt) = Self::generate_password(pwd);
        self.user_repo
            .save_with_permissions(
                &name,
                &email,
                &username,
                &pwd,
                &salt,
                vec!["Admin::Read".to_string(), "Admin::Write".to_string()],
            )
            .await
    }

    pub async fn update_password(&self, user_id: &Id, curr_password: String, new_password: String) -> AppResult<()> {
        //
        let ups = self.user_repo.get_password_by_id(user_id).await?;
        match Self::check_password(&curr_password, &ups.password, &ups.salt) {
            true => {
                let new_hash_pwd = Self::regenerate_password(new_password, ups.salt);
                self.user_repo.update_password(user_id, new_hash_pwd).await
            }
            false => Err(AppError::Unauthorized("wrong password".into())),
        }
    }

    fn generate_password(pwd: String) -> (String, String) {
        //
        // let salt: String = std::iter::repeat_with(fastrand::alphanumeric)
        // .take(12)
        // .collect();

        let salt = randoid!(12);
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
        (format!("{:x}", digest), salt)
    }

    fn regenerate_password(pwd: String, salt: String) -> String {
        //
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
        format!("{:x}", digest)
    }

    fn check_password(input_pwd: &str, pwd: &str, salt: &str) -> bool {
        //
        let digest = md5::compute(format!("@{salt}${input_pwd}").as_bytes());
        pwd == format!("{:x}", digest)
    }
}
