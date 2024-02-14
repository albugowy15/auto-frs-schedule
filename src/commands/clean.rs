use tokio::try_join;

use crate::db::{
    self,
    repository::{many_to_many::ManyToManyRepository, Repository},
};

pub async fn clean_handler() -> anyhow::Result<()> {
    println!("Open db connection...");
    let pool = db::Database::create_connection().await?;
    println!("Clean up invalid foreign key");
    let many_to_many_repo = ManyToManyRepository::new(&pool);

    try_join!(
        many_to_many_repo.drop_invalid_class_to_plan(),
        many_to_many_repo.drop_invalid_class_to_lecturer()
    )?;
    pool.close().await;
    println!("Done");
    Ok(())
}
