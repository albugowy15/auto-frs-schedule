use sqlx::{MySql, Pool};

use super::Repository;

pub struct ManyToManyRepository<'a> {
    db_pool: &'a Pool<MySql>,
}

impl<'a> Repository<'a> for ManyToManyRepository<'a> {
    fn new(db_pool: &'a Pool<MySql>) -> Self {
        ManyToManyRepository { db_pool }
    }
}

impl ManyToManyRepository<'_> {
    pub async fn drop_invalid_class_to_plan(&self) -> Result<(), sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;
        let result_b = sqlx::query("DELETE FROM _ClassToPlan WHERE B NOT IN (SELECT id FROM Plan)")
            .execute(&mut *tx)
            .await?;
        let result_a =
            sqlx::query("DELETE FROM _ClassToPlan WHERE A NOT IN (SELECT id from Class)")
                .execute(&mut *tx)
                .await?;
        let rows_affected = result_a.rows_affected() + result_b.rows_affected();
        println!("Deleted {} invalid rows from _ClassToPlan", rows_affected);
        tx.commit().await?;
        Ok(())
    }

    pub async fn drop_invalid_class_to_lecturer(&self) -> Result<(), sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;
        let result_a =
            sqlx::query("DELETE FROM _ClassToLecturer WHERE B NOT IN (SELECT id FROM Lecturer)")
                .execute(&mut *tx)
                .await?;
        let result_b =
            sqlx::query("DELETE FROM _ClassToLecturer WHERE A NOT IN (SELECT id from Class)")
                .execute(&mut *tx)
                .await?;
        let rows_affected = result_a.rows_affected() + result_b.rows_affected();
        println!("Deleted {} invalid rows _ClassToLecturer", rows_affected);
        tx.commit().await?;
        Ok(())
    }
}
