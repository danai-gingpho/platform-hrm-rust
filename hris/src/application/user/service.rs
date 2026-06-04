use crate::domain::entity::User;

pub struct UserService;

impl UserService {
    pub fn get_user(&self) -> User {
        User { id: 1, name: "John Doe".to_string() }
    }
}
