use anyhow::Result;
use sqlx::{MySql, Pool};

use crate::db::repository::{many_to_many_repository::ManyToManyRepository, Repository};

pub async fn clean_handler(pool: &Pool<MySql>) -> Result<()> {
    log::info!("Clean up invalid foreign key");
    let many_to_many_repo = ManyToManyRepository::new(pool);
    let clean_class_to_plan = many_to_many_repo.drop_invalid_class_to_plan();
    let clean_class_to_lecturer = many_to_many_repo.drop_invalid_class_to_lecturer();

    let result = tokio::try_join!(clean_class_to_plan, clean_class_to_lecturer);
    match result {
        Ok(_) => log::info!("Cleaning completed successfully"),
        Err(e) => log::error!("Cleaning failed: {}", e),
    }
    Ok(())
}
