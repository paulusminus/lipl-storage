// use anyhow::{anyhow, bail};
// use clap::{Arg, Command, command, value_parser, ArgAction, Subcommand, Parser};
use clap::{command, Subcommand, Parser};
use crate::repo::{RepoConfig};

// const SERVE: &str = "serve";
// const LIST: &str = "list";
// const COPY: &str = "copy";
// const PORT: &str = "port";
// const PORT_SHORT: char = 'p';
// const PORT_DEFAULT: &str = "3000";
// const SOURCE: &str = "source";
// const SOURCE_SHORT: char = 's';
// const TARGET: &str = "target";
// const TARGET_SHORT: char = 't';
// const YAML: &str = "yaml";
// const YAML_SHORT: char = 'y';

#[derive(Parser)]
struct ServeCommand {
    #[arg(long, short)]
    port: u16,
    #[arg(long, short)]
    source: RepoConfig,
}

#[derive(Parser)]
struct CopyCommand {
    #[arg(long, short)]
    source: RepoConfig,
    #[arg(long, short)]
    target: RepoConfig,
}

#[derive(Parser)]
struct ListCommand {
    #[arg(long, short)]
    source: RepoConfig,
    #[arg(long, short)]
    yaml: bool,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct LiplApp {
    #[command(subcommand)]
    command: LiplCommand,
}

#[derive(Subcommand)]
enum LiplCommand {
    Serve(ServeCommand),
    Copy(CopyCommand),
    List(ListCommand),
}

pub async fn run() -> anyhow::Result<()> {
    let cli = LiplApp::parse();
    match cli.command {
        LiplCommand::Serve(serve) => {
            let source_repo = serve.source.await?;
            crate::serve::run(source_repo, serve.port).await
        },
        LiplCommand::Copy(copy) => {
            let source_repo = copy.source.await?;
            let target_repo = copy.target.await?;
            crate::db::copy(source_repo, target_repo).await
        },
        LiplCommand::List(list) => {
            let source_repo = list.source.await?;
            crate::db::list(source_repo, list.yaml).await
        }
    }
}

// pub async fn run() -> anyhow::Result<()> {
//     let matches = command!()
//         .subcommand_required(true)
//         .subcommand(
//             Command::new(SERVE)
//                 .arg(
//                     Arg::new(PORT).short(PORT_SHORT).long(PORT).required(true).value_parser(value_parser!(u16)).default_value(PORT_DEFAULT)
//                 )
//                 .arg(
//                     Arg::new(SOURCE).short(SOURCE_SHORT).long(SOURCE).required(true)
//                 ),
//         )
//         .subcommand(
//             Command::new(COPY)
//                 .arg(
//                     Arg::new(SOURCE).short(SOURCE_SHORT).long(SOURCE).required(true)
//                 )
//                 .arg(
//                     Arg::new(TARGET).short(TARGET_SHORT).long(TARGET).required(true)
//                 )
//         )
//         .subcommand(
//             Command::new(LIST)
//                 .arg(
//                     Arg::new(SOURCE).short(SOURCE_SHORT).long(SOURCE).required(true)
//                 )
//                 .arg(
//                     Arg::new(YAML).short(YAML_SHORT).long(YAML).action(ArgAction::SetTrue)
//                 )
//         )
//         .get_matches();

//     match matches.subcommand() {
//         Some((SERVE, serve_matches)) => {
//             let port = serve_matches.get_one::<u16>(PORT).ok_or_else(|| anyhow!("Port missing"))?;
//             let source = serve_matches.get_one::<String>(SOURCE).ok_or_else(|| anyhow!("Source missing"))?;
//             let source_repo = source.parse::<RepoConfig>()?.await?;
//             crate::serve::run(source_repo, *port).await?;
//         },
//         Some((COPY, copy_matches)) => {
//             let source = copy_matches.get_one::<String>(SOURCE).ok_or_else(|| anyhow!("Source missing"))?;
//             let target = copy_matches.get_one::<String>(TARGET).ok_or_else(|| anyhow!("Target missing"))?;

//             let source_repo = source.parse::<RepoConfig>()?.await?;
//             let target_repo = target.parse::<RepoConfig>()?.await?;

//             crate::db::copy(source_repo, target_repo).await?;
//         },
//         Some((LIST, list_matches)) => {
//             let yaml = list_matches.get_one::<bool>(YAML).ok_or_else(|| anyhow!("Parsing yaml flag failed"))?;
//             let source = list_matches.get_one::<String>(SOURCE).ok_or_else(|| anyhow!("Source missing"))?;
//             let source_repo = source.parse::<RepoConfig>()?.await?;

//             crate::db::list(source_repo, *yaml).await?
//         },
//         Some((_, _)) => bail!("Unknown subcommand"),
//         None => bail!("Invalid parameters"),
//     }

//     Ok(())   
// }
