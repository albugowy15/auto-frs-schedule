use tokio::try_join;

use crate::db::{
    repository::{many_to_many::ManyToManyRepository, Repository},
    Database,
};

pub async fn clean_handler() {
    let pool = match Database::create_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            log::error!("Failed to create a db connection: {}", e);
            return;
        }
    };
    log::info!("Clean up invalid foreign key");
    let many_to_many_repo = ManyToManyRepository::new(&pool);

    if let Err(e) = try_join!(
        many_to_many_repo.drop_invalid_class_to_plan(),
        many_to_many_repo.drop_invalid_class_to_lecturer()
    ) {
        log::error!("Cleaning failed: {}", e);
        return;
    }
    pool.close().await;
}
