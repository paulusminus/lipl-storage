use anyhow::{anyhow, bail};
use clap::{Arg, Command, command, value_parser};
use lipl_fs_repo::{FileRepo, FileRepoConfig};
use lipl_postgres_repo::{PostgresRepo, PostgresRepoConfig};
use lipl_core::{LiplRepo, Summary, Lyric, Uuid, Playlist};

const SERVE: &str = "serve";
const LIST: &str = "list";
const COPY: &str = "copy";
const PORT: &str = "port";
const PORT_SHORT: char = 'p';
const PORT_DEFAULT: &str = "3000";
const SOURCE: &str = "source";
const SOURCE_SHORT: char = 's';
const TARGET: &str = "target";
const TARGET_SHORT: char = 't';
const YAML: &str = "yaml";
const YAML_SHORT: char = 'y';
const YAML_DEFAULT: &str = "false";

// #[derive(Clone)]
// enum RepoConfig {
//     Postgres(PostgresRepoConfig),
//     File(FileRepoConfig),
// }

#[derive(Clone)]
enum Repo {
    Postgres(PostgresRepo),
    File(FileRepo),
}

macro_rules! dispatch {
    ($self: ident, $method:ident $(,$param:expr)*) => {
        match $self {
            Repo::File(file) => file.$method($($param),*).await,
            Repo::Postgres(postgres) => postgres.$method($($param)*).await
        }        
    };
}

#[async_trait::async_trait]
impl LiplRepo for Repo {
    async fn get_lyrics(&self) -> anyhow::Result<Vec<Lyric>> {
        dispatch!(self, get_lyrics)
    }

    async fn get_lyric_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        dispatch!(self, get_lyric_summaries)
    }

    async fn get_lyric(&self, id: Uuid) -> anyhow::Result<Lyric> {
        dispatch!(self, get_lyric, id)
    }

    async fn post_lyric(&self, lyric: Lyric) -> anyhow::Result<Lyric> {
        dispatch!(self, post_lyric, lyric)
    }

    async fn delete_lyric(&self, id: Uuid) -> anyhow::Result<()> {
        dispatch!(self, delete_lyric, id)
    }

    async fn get_playlists(&self) -> anyhow::Result<Vec<Playlist>> {
        dispatch!(self, get_playlists)
    }

    async fn get_playlist_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        dispatch!(self, get_playlist_summaries)
    }

    async fn get_playlist(&self, id: Uuid) -> anyhow::Result<Playlist> {
        dispatch!(self, get_playlist, id)
    }

    async fn post_playlist(&self, playlist: Playlist) -> anyhow::Result<Playlist> {
        dispatch!(self, post_playlist, playlist)
    }

    async fn delete_playlist(&self, id: Uuid) -> anyhow::Result<()> {
        dispatch!(self, delete_playlist, id)
    }

    async fn stop(&self) -> anyhow::Result<()> {
        dispatch!(self, stop)
    }
}

async fn try_repo_from(s: &String) -> anyhow::Result<Repo> {
    let splitted = s.split(':').collect::<Vec<&str>>();
    if splitted.len() == 2 {
        let repo_dir = splitted[1].to_owned();
        if splitted[0] == "file" {
            repo_dir.parse::<FileRepoConfig>()?.await.map(Repo::File)
        }
        else if splitted[0] == "postgres" {
            repo_dir.parse::<PostgresRepoConfig>()?.await.map(Repo::Postgres)
        }
        else {
            bail!("Unknown prefix for db connection string")
        }
    }
    else {
        bail!("Problem with separator (none or too many)")
    }
}

pub async fn run() -> anyhow::Result<()> {
    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new(SERVE)
                .arg(
                    Arg::new(PORT).short(PORT_SHORT).long(PORT).required(true).value_parser(value_parser!(u16)).default_value(PORT_DEFAULT)
                )
                .arg(
                    Arg::new(SOURCE).short(SOURCE_SHORT).long(SOURCE).required(true)
                ),
        )
        .subcommand(
            Command::new(COPY)
                .arg(
                    Arg::new(SOURCE).short(SOURCE_SHORT).long(SOURCE).required(true)
                )
                .arg(
                    Arg::new(TARGET).short(TARGET_SHORT).long(TARGET).required(true)
                )
        )
        .subcommand(
            Command::new(LIST)
                .arg(
                    Arg::new(SOURCE).short(SOURCE_SHORT).long(SOURCE).required(true)
                )
                .arg(
                    Arg::new(YAML).short(YAML_SHORT).long(YAML).value_parser(value_parser!(bool)).default_value(YAML_DEFAULT)
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some((SERVE, serve_matches)) => {
            let port = serve_matches.get_one::<u16>("port").ok_or(anyhow!("Port missing"))?;
            let source = serve_matches.get_one::<String>(SOURCE).ok_or(anyhow!("Source missing"))?;
            let source_repo = try_repo_from(source).await?;
            crate::serve::run(source_repo, port.clone()).await?;
        },
        Some((COPY, copy_matches)) => {
            let source = copy_matches.get_one::<String>(SOURCE).ok_or(anyhow!("Source missing"))?;
            let target = copy_matches.get_one::<String>(TARGET).ok_or(anyhow!("Target missing"))?;

            let source_repo = try_repo_from(source).await?;
            let target_repo = try_repo_from(target).await?;

            crate::db::copy(source_repo, target_repo).await?;
        },
        Some((LIST, list_matches)) => {
            let yaml = list_matches.get_one::<bool>(YAML).ok_or(anyhow!("Parsing yaml flag failed"))?;
            let source = list_matches.get_one::<String>(SOURCE).ok_or(anyhow!("Source missing"))?;
            let source_repo = try_repo_from(source).await?;

            crate::db::list(source_repo, yaml.clone()).await?
        },
        Some((_, _)) => bail!("Unknown subcommand"),
        None => bail!("Invalid parameters"),
    }

    Ok(())
    
}