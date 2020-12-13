use std::collections::HashMap;
use std::fs::File;
use std::io::{Write};
use std::path::Path;

use zip::read::{ZipArchive};

use crate::model::{parts_to_string, PathBufExt, LiplResult, Lyric, Playlist, PlaylistPost, Uuid, UuidExt, TXT, YAML};
use crate::io::{get_lyric, get_playlist};

pub fn zip_read<P>(path: P) -> LiplResult<(HashMap<Uuid, Lyric>, HashMap<Uuid, Playlist>)>
where P: AsRef<Path> {
    let zip_file = File::open(path)?;
    let zip = &mut ZipArchive::new(zip_file)?;

    let mut lyric_hm: HashMap<Uuid, Lyric> = HashMap::new();
    let mut playlist_hm: HashMap<Uuid, Playlist> = HashMap::new();

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        let uuid = (&file.name()).to_uuid();
        if file.is_file() && file.name().has_extension(TXT) {
            lyric_hm.insert(
                uuid,
                get_lyric(file).map(|lp| Lyric::from((Some(uuid), lp)))?
            );
        }
        else if file.is_file() && file.name().has_extension(YAML) {
            playlist_hm.insert(
                uuid,
                get_playlist(file).map(|pp| Playlist::from((Some(uuid), pp)))?
            );
        }
    };
    
    Ok((lyric_hm, playlist_hm))
}

pub fn zip_write<P: AsRef<Path>>(path: P, lyrics: HashMap<Uuid, Lyric>, playlists: HashMap<Uuid, Playlist>) -> LiplResult<()> {
    let file = File::create(path)?;
    let zip = &mut zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    zip.set_comment("Lipl Database");

    for lyric in lyrics.values() {
        let filename = format!("{}.{}", lyric.id.to_base58(), TXT);
        let title_content = lyric.title.as_ref().map(|s| format!("---\ntitle: {}\n---\n\n", s)).unwrap_or_default();
        let content = format!("{}{}", title_content, parts_to_string(&lyric.parts));
        let bytes = content.as_str().as_bytes();
        zip.start_file(&filename, options)?;
        zip.write_all(bytes)?;
    };

    for playlist in playlists.values() {
        let filename = format!("{}.{}", playlist.id.to_base58(), YAML);
        let disk_playlist = PlaylistPost::from((playlist.title.clone(), playlist.members.clone()));
        let content = serde_yaml::to_string(&disk_playlist)?;
        let bytes = content.as_str().as_bytes();
        zip.start_file(&filename, options)?;
        zip.write_all(bytes)?;
    }

    zip.finish()?;
    
    Ok(())
}
