use crate::modules::auth::domain::user::user::UserRepository;

pub trait HasUserRepo {
    fn user(&mut self) -> impl UserRepository + '_;
}
