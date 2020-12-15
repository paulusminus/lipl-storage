use std::fs::File;
use std::io::{Write};
use std::path::Path;

use zip::read::{ZipArchive};

use crate::model::{parts_to_string, PathBufExt, LiplResult, Lyric, Playlist, PlaylistPost, UuidExt, TXT, YAML, Db};
use crate::io::{get_lyric, get_playlist};

pub fn zip_read<P>(path: P) -> LiplResult<Db>
where P: AsRef<Path> {
    let zip_file = File::open(&path)?;
    let mut zip = ZipArchive::new(zip_file)?;

    let mut db = Db::new(path.as_ref().into());

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        let uuid = (&file.name()).to_uuid();
        if file.name().is_file_type(TXT) {
            db.add_lyric(
                get_lyric(file).map(|lp| Lyric::from((Some(uuid), lp)))?
            );
        }
    }

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        let uuid = (&file.name()).to_uuid();
        if file.name().is_file_type(YAML) {
            db.add_playlist(
                get_playlist(file).map(|pp| Playlist::from((Some(uuid), pp)))?
            );
        }
    };
    
    Ok(db)
}

pub fn zip_write<P: AsRef<Path>>(path: P, db: Db) -> LiplResult<()> {
    let file = File::create(path)?;
    let zip = &mut zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    zip.set_comment("Lipl Database");

    for lyric in db.get_lyric_list() {
        let filename = format!("{}.{}", lyric.id.to_base58(), TXT);
        let title_content = lyric.title.as_ref().map(|s| format!("---\ntitle: {}\n---\n\n", s)).unwrap_or_default();
        let content = format!("{}{}", title_content, parts_to_string(&lyric.parts));
        let bytes = content.as_str().as_bytes();
        zip.start_file(&filename, options)?;
        zip.write_all(bytes)?;
    };

    for playlist in db.get_playlist_list() {
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
