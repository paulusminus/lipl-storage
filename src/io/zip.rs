use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{Write};
use uuid::Uuid;
use zip::{ZipArchive};
use zip::read::ZipFile;
use crate::model;
use crate::io;
use model::{UuidExt, PathBufExt, LiplResult, Lyric, Playlist};

fn to_uuid(z: &ZipFile) -> Uuid {
    z.name().to_uuid()
}

pub fn zip_read<P>(path: P) -> LiplResult<(HashMap<Uuid, Lyric>, HashMap<Uuid, Playlist>)>
where P: AsRef<Path> {
    let zip_file = File::open(path)?;
    let zip = &mut ZipArchive::new(zip_file)?;

    let mut lyric_hm: HashMap<Uuid, model::Lyric> = HashMap::new();
    let mut playlist_hm: HashMap<Uuid, model::Playlist> = HashMap::new();

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        let uuid = to_uuid(&file);
        if file.is_file() && file.name().ends_with(".txt") {
            lyric_hm.insert(
                uuid,
                io::get_lyric(file, uuid)?
            );
        }
        else if file.is_file() && file.name().ends_with(".yaml") {
            playlist_hm.insert(
                uuid,
                (uuid, io::get_playlist(file).unwrap()).into()
            );
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

pub fn zip_write<P: AsRef<Path>>(path: P, lyrics: HashMap<Uuid, Lyric>, playlists: HashMap<Uuid, Playlist>) -> LiplResult<()> {
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
