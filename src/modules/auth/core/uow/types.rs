use crate::modules::auth::core::domain::user::UserRepository;

pub trait HasUserRepo {
    fn user(&mut self) -> impl UserRepository + '_;
}
