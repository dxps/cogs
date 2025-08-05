//! `Id` and `UserAccount` models are also defined here, besides being in `shared` crate.
//! The reason is that without them being defined within this crate,
//! `impl Authentication<UserAccount, Id, PgPool> for UserAccount` wouldn't be possible.

mod id;
pub use id::*;

mod user;
pub use user::*;
