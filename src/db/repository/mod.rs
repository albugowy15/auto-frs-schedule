use sqlx::{MySql, Pool};

pub mod class;
pub mod lecturer;
pub mod many_to_many;
pub mod plan;
pub mod session;
pub mod subject;

pub trait Repository<'a> {
    fn new(db_pool: &'a Pool<MySql>) -> Self;
}
