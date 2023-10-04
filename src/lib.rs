
pub mod auth;
pub mod health;
pub mod hash;
pub mod users;
pub mod db;

pub mod features;
pub mod projects;
pub mod external;

pub mod models;

pub mod gen {
    pub fn id() -> String {
        uuid::Uuid::new_v4().to_string()
    }
}