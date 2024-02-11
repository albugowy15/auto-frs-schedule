use tokio::try_join;

use crate::db::{
    self,
    repository::{many_to_many::ManyToManyRepository, Repository},
};

pub async fn clean_handler() {
    let pool = match db::Database::create_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            log::error!("{}", e);
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
