use crate::db::{
    self,
    repository::{class::ClassRepository, plan::PlanRepository, Repository},
};

pub async fn sync_handler() -> anyhow::Result<()> {
    let pool = db::Database::create_connection().await?;
    println!("Sync totalSks from Plan...");
    PlanRepository::new(&pool).sync_total_sks().await?;
    println!("Sync taken from Class..");
    ClassRepository::new(&pool).sync_taken().await?;
    pool.close().await;
    Ok(())
}
