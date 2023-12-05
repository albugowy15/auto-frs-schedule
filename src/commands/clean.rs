use sqlx::{MySql, Pool};

use crate::db::repository::{many_to_many_repository::ManyToManyRepository, Repository};

pub async fn clean_handler(pool: &Pool<MySql>) {
    log::info!("Clean up invalid foreign key");
    let many_to_many_repo = ManyToManyRepository::new(pool);

    if let Err(e) = tokio::try_join!(
        many_to_many_repo.drop_invalid_class_to_plan(),
        many_to_many_repo.drop_invalid_class_to_lecturer()
    ) {
        log::error!("Cleaning failed: {}", e);
    }
}
