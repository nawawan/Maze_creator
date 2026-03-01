use crate::repository::base_repository::BaseRepository;
use crate::repository::blog::BlogRepository;
use crate::repository::user::UserRepository;

pub trait Repositories: BaseRepository + BlogRepository + UserRepository {}
