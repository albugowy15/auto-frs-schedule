use sqlx::{MySql, Pool};

pub mod class_repository;
pub mod lecturer_repository;
pub mod many_to_many_repository;
pub mod session_repository;
pub mod subject_repository;

pub trait Repository<'b> {
    fn new(db_pool: &Pool<MySql>) -> &'b Self;
}
