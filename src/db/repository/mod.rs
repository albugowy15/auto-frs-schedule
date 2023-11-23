use sqlx::{MySql, Pool};

pub mod class_repository;
pub mod lecturer_repository;
pub mod many_to_many_repository;
pub mod plan_repository;
pub mod session_repository;
pub mod subject_repository;

pub trait Repository<'a> {
    fn new(db_pool: &'a Pool<MySql>) -> Self;
}
