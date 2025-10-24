use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

pub type UserStore = Arc<RwLock<HashMap<Uuid, User>>>;
pub type EmailIndex = Arc<RwLock<HashMap<String, Uuid>>>;

#[derive(Clone)]
pub struct MemoryStore {
    pub users: UserStore,
    pub email_index: EmailIndex,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            email_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_user(&self, user: User) -> Result<User, String> {
        let mut users = self.users.write().await;
        let mut email_index = self.email_index.write().await;

        // 检查邮箱是否已存在
        if email_index.contains_key(&user.email) {
            return Err("用户邮箱已存在".to_string());
        }

        // 检查用户名是否已存在
        for existing_user in users.values() {
            if existing_user.username == user.username {
                return Err("用户名已存在".to_string());
            }
        }

        let user_id = user.id;
        let email = user.email.clone();
        
        users.insert(user_id, user.clone());
        email_index.insert(email, user_id);

        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Option<User> {
        let users = self.users.read().await;
        users.get(&id).cloned()
    }

    pub async fn get_user_by_email(&self, email: &str) -> Option<User> {
        let email_index = self.email_index.read().await;
        if let Some(user_id) = email_index.get(email) {
            let users = self.users.read().await;
            users.get(user_id).cloned()
        } else {
            None
        }
    }

    pub async fn get_all_users(&self, page: u32, size: u32) -> (Vec<User>, u32) {
        let users = self.users.read().await;
        let total = users.len() as u32;
        let start = ((page - 1) * size) as usize;
        let _end = (start + size as usize).min(users.len());

        let mut user_list: Vec<User> = users.values().cloned().collect();
        user_list.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let paginated_users = user_list.into_iter().skip(start).take(size as usize).collect();
        (paginated_users, total)
    }

    pub async fn delete_user(&self, id: Uuid) -> bool {
        let mut users = self.users.write().await;
        let mut email_index = self.email_index.write().await;

        if let Some(user) = users.remove(&id) {
            email_index.remove(&user.email);
            true
        } else {
            false
        }
    }
}
