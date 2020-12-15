use std::path::{Path};
use std::time::{Instant};
use crate::model::{LiplResult, PathBufExt, ZIP};
use crate::io::{fs_read, fs_write, zip_read, zip_write};

pub fn copy<P: AsRef<Path>>(source: P, target: P) -> LiplResult<()> {
    let start = Instant::now();

    let (lyrics, playlists) = if source.is_file_type(ZIP) { zip_read(source)? } else { fs_read(source)? };

    if target.is_file_type(ZIP) { 
        zip_write(target, lyrics, playlists)?
    }
    else {
        fs_write(target, lyrics, playlists)?;
    };

    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}
