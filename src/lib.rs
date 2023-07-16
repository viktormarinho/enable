use std::{sync::Arc, collections::HashMap};

use tokio::sync::Mutex;

pub mod auth;
pub mod health;
pub mod hash;

pub type SessionsStore = Arc<Mutex<HashMap<String, u64>>>;