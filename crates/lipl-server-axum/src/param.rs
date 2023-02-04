#[cfg(feature = "postgres")]
pub mod app {
    use std::sync::Arc;
    use async_trait::async_trait;
    use clap::{ArgGroup, Parser};
    use lipl_core::{LiplRepo, ToRepo};
    use lipl_repo_memory::MemoryRepoConfig;

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    #[command(group(ArgGroup::new("db").required(true).args(["postgres", "memory"])))]
    pub struct LiplApp {
        #[arg(long, group = "db")]
        pub postgres: Option<String>,
        #[arg(long, group = "db")]
        pub memory: Option<bool>,
    }

    impl LiplApp {
        pub fn new_memory(memory: bool) -> Self {
            Self { postgres: None, memory: Some(memory)}
        }
    }

    #[async_trait]
    impl ToRepo for LiplApp {
    async fn to_repo(self) -> lipl_core::Result<Arc<dyn LiplRepo>> {
        if let Some(postgres) = self.postgres {
            let pool = lipl_axum_postgres::connection_pool(&postgres).await?;
            Ok(
                Arc::new(pool)
            )    
    
        }
        else {
            let memory = self.memory.unwrap();
            MemoryRepoConfig { include_sample_data: memory }
                .to_repo()
                .await
        }
    }
}


}

#[cfg(not(feature = "postgres"))]
pub mod app {
    use std::{sync::Arc};
    use async_trait::async_trait;
    use clap::{Parser};
    use lipl_core::{LiplRepo, ToRepo};

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    pub struct LiplApp {
        #[arg(long)]
        pub memory: bool,
    }    

    impl LiplApp {
        pub fn new_memory(include_sample_data: bool) -> Self {
            Self {
                memory: include_sample_data,
            }
        }    
    }

    #[async_trait]
    impl ToRepo for LiplApp {
    async fn to_repo(self) -> lipl_core::Result<Arc<dyn LiplRepo>> {
        lipl_repo_memory::MemoryRepoConfig { sample_data: self.memory, transaction_log: None }
            .to_repo()
            .await
        }
    }
}
