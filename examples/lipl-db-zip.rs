use std::fs::File;
use std::io::Write;
use std::time::{Instant};
use tokio::runtime::{Builder};

use lipl_io::{create_db, get_path, UuidExt};

fn parts_to_string(parts: &Vec<Vec<String>>) -> String {
    parts
    .iter()
    .map(|part| part.join("\n"))
    .collect::<Vec<String>>()
    .join("\n\n")
}

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let result = rt.block_on(async {
        let path = get_path()?;

        let file = File::create("./out/lipl.zip")?;
        let zip = &mut zip::ZipWriter::new(file);
        let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        zip.set_comment("Lipl Database");

        let (lyrics, playlists) = create_db(&path).await?;

        for lyric in lyrics.values() {
            let filename = format!("{}.txt", lyric.id.to_base58());
            let title_content = lyric.title.as_ref().map(|s| format!("---\ntitle: {}\n---\n\n", s)).unwrap_or_default();
            let content = format!("{}{}", title_content, parts_to_string(&lyric.parts));
            let bytes = content.as_str().as_bytes();
            zip.start_file(&filename, options)?;
            zip.write_all(bytes)?;
        };

        for playlist in playlists.values() {
            let filename = format!("{}.yaml", playlist.id.to_base58());
            let disk_playlist = lipl_io::PlaylistPost::from((playlist.title.clone(), playlist.members.clone()));
            let content = serde_yaml::to_string(&disk_playlist).unwrap();
            let bytes = content.as_str().as_bytes();
            zip.start_file(&filename, options)?;
            zip.write_all(bytes)?;
        }

        zip.finish()?;
    
        Ok(())
    });

    println!("Elapsed: {:?}", start.elapsed());
    result
}
