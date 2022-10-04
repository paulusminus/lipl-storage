use clap::{Parser};

#[derive(Debug, Parser)]
#[clap(name = "lipl-upload", author, version, about)]
pub struct Args {
    #[clap(short, long, required = true, help = "Path where the lyric files are stored on disk")]
    pub source_path: String,
    #[clap(short, long, required = true, help = "API Prefix")]
    pub prefix: String,
    #[clap(short, long, required = true, help = "File with this extension is to be uploaded")]
    pub filter: String,
    #[clap(required = true, help = "Sets the name of the playlist where uploaded lyrics are to be made member of")]
    pub playlist_name: String,
}
