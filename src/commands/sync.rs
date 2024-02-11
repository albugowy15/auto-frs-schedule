use std::sync::Arc;

use sqlx::MySqlPool;

use crate::{
    commands::create_db_connection,
    db::repository::{class::ClassRepository, plan::PlanRepository, Repository},
};

fn sync_taken(pool: &Arc<MySqlPool>) -> tokio::task::JoinHandle<()> {
    let cloned_pool = pool.clone();
    tokio::task::spawn(async move {
        log::info!("Sync taken from Class");
        ClassRepository::new(&cloned_pool)
            .sync_taken()
            .await
            .unwrap();
    })
}

fn sync_total_sks(pool: &Arc<MySqlPool>) -> tokio::task::JoinHandle<()> {
    let cloned_pool = pool.clone();
    tokio::task::spawn(async move {
        log::info!("Sync totalSks from Plan");
        PlanRepository::new(&cloned_pool)
            .sync_total_sks()
            .await
            .unwrap();
    })
}

pub async fn sync_handler() {
    let pool = Arc::new(create_db_connection().await.unwrap());

    if let Err(e) = tokio::try_join!(sync_taken(&pool), sync_total_sks(&pool)) {
        log::error!("Error syncing: {}", e);
        return;
    }
    pool.close().await;
    log::info!("Successfully synced taken and totalSks from Class and Plan tables");
}
