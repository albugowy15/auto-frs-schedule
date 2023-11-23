use anyhow::Result;
use sqlx::{MySql, Pool};

use crate::db::repository::{
    class_repository::ClassRepository, plan_repository::PlanRepository, Repository,
};

pub async fn sync_handler(pool: &Pool<MySql>) -> Result<()> {
    log::info!("Sync taken from Class");
    ClassRepository::new(pool).sync_taken().await?;

    log::info!("Sync totalSks from Plan");
    PlanRepository::new(pool).sync_total_sks().await?;
    Ok(())
}
