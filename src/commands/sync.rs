use anyhow::Result;
use sqlx::{MySql, Pool};

use crate::db::repository::{
    class_repository::ClassRepository, plan_repository::PlanRepository, Repository,
};

pub async fn sync_handler(pool: &Pool<MySql>) -> Result<()> {
    log::info!("Sync taken from Class");
    let class_repo = ClassRepository::new(pool);

    log::info!("Sync totalSks from Plan");
    let plan_repo = PlanRepository::new(pool);

    match tokio::try_join!(class_repo.sync_taken(), plan_repo.sync_total_sks()) {
        Ok(_) => log::info!("Succesfully sync taken and totalSks from Class and Plan table"),
        Err(e) => log::error!("Error sync : {}", e),
    }
    Ok(())
}
