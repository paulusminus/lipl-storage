use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;
use uuid::Uuid;
use zip::{ZipArchive};
use zip::read::ZipFile;
use crate::model;
use crate::io;
use model::{UuidExt, PathBufExt};

fn to_uuid(z: &ZipFile) -> Uuid {
    PathBuf::from(z.name()).to_uuid()
}

fn filename(z: &ZipFile) -> String {
    z.name().into()
}

pub async fn zip_read(path: &str) -> Result<(HashMap<Uuid, model::Lyric>, HashMap<Uuid, model::Playlist>), Error> {
    let zip_file = File::open(path)?;
    let zip = &mut ZipArchive::new(zip_file)?;

    let mut lyric_hm: HashMap<Uuid, model::Lyric> = HashMap::new();
    let mut playlist_hm: HashMap<Uuid, model::Playlist> = HashMap::new();

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        if file.is_file() {
            let uuid = to_uuid(&file);
            let filename = filename(&file);
            if filename.ends_with(".txt") {
                lyric_hm.insert(
                    uuid,
                    io::get_lyric(file, uuid).await?
                );
            }
            else if filename.ends_with(".yaml") {
                playlist_hm.insert(
                    uuid,
                    (uuid, io::get_playlist(Ok(file)).unwrap()).into()
                );
            }
        }
    };
    Ok((lyric_hm, playlist_hm))
}

fn parts_to_string(parts: &Vec<Vec<String>>) -> String {
    parts
    .iter()
    .map(|part| part.join("\n"))
    .collect::<Vec<String>>()
    .join("\n\n")
}

pub fn zip_write(path: &str, lyrics: HashMap<Uuid, model::Lyric>, playlists: HashMap<Uuid, model::Playlist>) -> Result<(), std::io::Error> {


    let file = File::create(path)?;
    let zip = &mut zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    zip.set_comment("Lipl Database");

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
        let disk_playlist = model::PlaylistPost::from((playlist.title.clone(), playlist.members.clone()));
        let content = serde_yaml::to_string(&disk_playlist).unwrap();
        let bytes = content.as_str().as_bytes();
        zip.start_file(&filename, options)?;
        zip.write_all(bytes)?;
    }

    zip.finish()?;
    
    Ok(())
}

