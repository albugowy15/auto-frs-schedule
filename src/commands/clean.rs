use anyhow::Result;
use sqlx::{MySql, Pool};

use crate::db::repository::many_to_many_repository::ManyToManyRepository;

pub async fn clean_handler(pool: &Pool<MySql>) -> Result<()> {
    log::info!("Clean up invalid foreign key");
    let many_to_many_repo = ManyToManyRepository::new(pool);
    log::info!("Cleaning _ClassToPlan");
    many_to_many_repo.drop_invalid_class_to_plan().await?;
    log::info!("Cleaning _ClassToLecturer");
    many_to_many_repo.drop_invalid_class_to_lecturer().await?;
    Ok(())
}
