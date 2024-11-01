use async_sqlx_session::MySqlSessionStore;
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    MySqlPool,
};

#[derive(Debug)]
pub struct Repository {
    pool: MySqlPool,
    session_store: MySqlSessionStore,
}

impl Repository {
    pub async fn connect() -> anyhow::Result<Self> {
        let pool_options = get_pool_options_from_env()?;

        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect_with(pool_options)
            .await?;

        let session_store =
            MySqlSessionStore::from_client(pool.clone()).with_table_name("user_sessions");

        Ok(Self {
            pool,
            session_store,
        })
    }

    pub async fn migrate(&self) -> anyhow::Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;

        Ok(())
    }
}

fn get_pool_options_from_env() -> anyhow::Result<MySqlConnectOptions> {
    let host = std::env::var("NS_MARIADB_HOSTNAME")?;
    let port = std::env::var("NS_MARIADB_PORT")?
        .parse()
        .map_err(|_| anyhow::anyhow!("DB_PORT must be a number"))?;
    let user = std::env::var("NS_MARIADB_USER")?;
    let password = std::env::var("NS_MARIADB_PASSWORD")?;
    let db_name = std::env::var("NS_MARIADB_DATABASE")?;

    Ok(MySqlConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&user)
        .password(&password)
        .database(&db_name))
}
