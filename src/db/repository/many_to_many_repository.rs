use anyhow::Result;
use sqlx::{MySql, Pool};

pub struct ManyToManyRepository<'a> {
    db_pool: &'a Pool<MySql>,
}

impl ManyToManyRepository<'_> {
    pub fn new(db_pool: &Pool<MySql>) -> ManyToManyRepository {
        ManyToManyRepository { db_pool }
    }

    pub async fn drop_invalid_class_to_plan(&self) -> Result<()> {
        let mut tx = self.db_pool.begin().await?;
        let result_b = sqlx::query("DELETE FROM _ClassToPlan WHERE B NOT IN (SELECT id FROM Plan)")
            .execute(&mut *tx)
            .await?;

        let result_a =
            sqlx::query("DELETE FROM _ClassToPlan WHERE A NOT IN (SELECT id from Class)")
                .execute(&mut *tx)
                .await?;
        log::info!(
            "Cleaned {} invalid _ClassToPlan",
            result_a.rows_affected() + result_b.rows_affected()
        );
        tx.commit().await?;
        Ok(())
    }

    pub async fn drop_invalid_class_to_lecturer(&self) -> Result<()> {
        let mut tx = self.db_pool.begin().await?;
        let result_a =
            sqlx::query("DELETE FROM _ClassToLecturer WHERE B NOT IN (SELECT id FROM Lecturer)")
                .execute(&mut *tx)
                .await?;
        let result_b =
            sqlx::query("DELETE FROM _ClassToLecturer WHERE A NOT IN (SELECT id from Class)")
                .execute(&mut *tx)
                .await?;
        log::info!(
            "Cleaned {} invalid _ClassToLecturer",
            result_a.rows_affected() + result_b.rows_affected()
        );
        tx.commit().await?;
        Ok(())
    }
}
