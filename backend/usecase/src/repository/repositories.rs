use crate::repository::blog::BlogRepository;
use crate::repository::user::UserRepository;

pub trait Reporisories: BlogRepository + UserRepository {}