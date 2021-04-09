use crate::config::CONFIG;
use once_cell::sync::OnceCell;
use sqlx::sqlite::SqlitePool;

static SQLITE_POOL: OnceCell<SqlitePool> = OnceCell::new();

pub struct DBPool {}

impl DBPool {
    pub async fn create_pool() -> SqlitePool {
        SqlitePool::connect(&CONFIG.db_url).await.unwrap()
    }

    pub fn init_pool(pool: SqlitePool) {
        assert!(SQLITE_POOL.set(pool).is_ok());
    }

    pub fn get_pool() -> &'static SqlitePool {
        SQLITE_POOL.get().unwrap()
    }
}

#[cfg(test)]
mod test {}

pub trait SqlGen {
    fn table_name() -> String;
    fn column_kes() -> String;
}
