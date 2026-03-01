use crate::repository::blog::BlogRepository;
use crate::repository::user::UserRepository;
use crate::repository::base_repository::BaseRepository;

pub trait Repositories: BaseRepository + BlogRepository + UserRepository {}