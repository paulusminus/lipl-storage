use clap::{Parser};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, required = true, help = "Path where the lyric files are stored on disk")]
    pub source_path: String,
    #[arg(short, long, required = true, help = "API Prefix")]
    pub prefix: String,
    #[arg(short, long, required = true, help = "File with this extension is to be uploaded")]
    pub filter: String,
    #[arg(required = true, help = "Sets the name of the playlist where uploaded lyrics are to be made member of")]
    pub playlist_name: String,
}
