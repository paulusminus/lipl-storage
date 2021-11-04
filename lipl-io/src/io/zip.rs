use std::fs::File;
use std::io::{Write};
use std::path::{Path};

use log::{info};
use zip::read::{ZipArchive};

use crate::model::{LiplResult, TXT, YAML, Db, HasExtension, HasId, ToDiskFormat, ExtractUuid};
use crate::io::{lyricpost_from_reader, playlistpost_from_reader};

pub fn zip_read<P>(path: P, db: &mut Db) -> LiplResult<()>
where P: AsRef<Path> {
    info!("Starting to read from zip file {}", path.as_ref().to_string_lossy());
    let zip_file = File::open(path.as_ref())?;
    let mut zip = ZipArchive::new(zip_file)?;

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        if file.mangled_name().has_extension(YAML) {
            let uuid = file.mangled_name().extract_uuid()?;
            info!("Adding playlist: {}", &file.name());
            db.add_playlist(
                &mut playlistpost_from_reader(file).map(|pp| (Some(uuid), pp).into())?
            );
        }
        else if file.mangled_name().has_extension(TXT) {
            let uuid = file.mangled_name().extract_uuid()?;
            info!("Adding lyric: {}", &file.name());
            db.add_lyric(
                &lyricpost_from_reader(file).map(|lp| (Some(uuid), lp).into())?
            );
        }
    };
    
    Ok(())
}

fn write_zip_item<T>(item: T, ext: &str, zip: &mut zip::ZipWriter<std::fs::File>) -> LiplResult<()> where T: HasId + ToDiskFormat {
    let filename = format!("{}.{}", item.id().to_string(), ext);
    info!("Writing: {}", &filename);
    let content = item.to_disk_format()?;
    let bytes = content.as_str().as_bytes();

    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    zip.start_file(&filename, options)?;
    zip.write_all(bytes)?;

    Ok(())
}

pub fn zip_write<P>(path: P, db: &Db) -> LiplResult<()> 
where P: AsRef<Path>
{
    info!("Starting to write to zip file {}", path.as_ref().to_string_lossy());

    let file = File::create(path)?;
    let zip = &mut zip::ZipWriter::new(file);
    zip.set_comment("Lipl Database");

    for lyric in db.get_lyric_list() {
        write_zip_item(
            lyric.clone(),
            TXT,
            zip,
        )?;
    }

    for playlist in db.get_playlist_list() {
        write_zip_item(
            playlist.clone(),
            YAML,
            zip,
        )?;
    }

    zip.finish()?;
    
    Ok(())
}
