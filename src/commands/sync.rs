use sqlx::{MySql, Pool};

use crate::db::repository::{
    class_repository::ClassRepository, plan_repository::PlanRepository, Repository,
};

pub async fn sync_handler(pool: &Pool<MySql>) {
    log::info!("Sync taken from Class");
    let class_repo = ClassRepository::new(pool);

    log::info!("Sync totalSks from Plan");
    let plan_repo = PlanRepository::new(pool);

    if let Err(e) = tokio::try_join!(class_repo.sync_taken(), plan_repo.sync_total_sks()) {
        log::error!("Error syncing: {}", e);
        return;
    }

    log::info!("Successfully synced taken and totalSks from Class and Plan tables");
}
