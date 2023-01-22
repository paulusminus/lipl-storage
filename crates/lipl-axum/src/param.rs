use std::sync::Arc;
use async_trait::async_trait;
use clap::Parser;
use lipl_core::{ToRepo, LiplRepo};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct LiplApp {
    #[arg(long)]
    pub postgres: bool,
}

#[cfg(feature = "postgres")]
#[async_trait]
impl ToRepo for LiplApp {
    async fn to_repo(self) -> lipl_core::Result<Arc<dyn LiplRepo>> {
        if self.postgres {
            let pool = lipl_axum_postgres::connection_pool(crate::constant::PG_CONNECTION).await?;
            Ok(
                Arc::new(pool)
            )    
    
        }
        else {
            Ok(
                Arc::new(
                    lipl_repo_memory::MemoryRepo::default(),
                )
            )   
        }
    }
}

#[cfg(not(feature = "postgres"))]
#[async_trait]
impl ToRepo for LiplApp {
    async fn to_repo(self) -> lipl_core::Result<Arc<dyn LiplRepo>> {
        Ok(
            Arc::new(
                lipl_repo_memory::MemoryRepo::default(),
            )
        )   
    }
}
