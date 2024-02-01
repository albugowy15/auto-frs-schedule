use anyhow::Result;
use sqlx::{MySql, Pool, Row};

use super::Repository;

pub struct PlanRepository<'a> {
    db_pool: &'a Pool<MySql>,
}

impl<'a> Repository<'a> for PlanRepository<'a> {
    fn new(db_pool: &'a Pool<MySql>) -> Self {
        PlanRepository { db_pool }
    }
}

impl PlanRepository<'_> {
    pub async fn sync_total_sks(&self) -> Result<()> {
        let mut tx = self.db_pool.begin().await?;
        let rows = sqlx::query(
            "select p.id, p.totalSks, sum(m.sks) as actual_sks from Plan p inner join _ClassToPlan cp on cp.B = p.id inner join Class c on cp.A = c.id inner join Matkul m on c.matkulId = m.id group by p.id having p.totalSks != sum(m.sks)",
        )
        .fetch_all(&mut *tx)
        .await?;
        log::info!("Sync totalSks {} plans", rows.len());

        for row in rows.into_iter() {
            let actual_sks: i8 = row.get("actual_sks");
            let plan_id: String = row.get("id");
            sqlx::query("update Plan set totalSks = ? where id = ?")
                .bind(actual_sks)
                .bind(plan_id)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
