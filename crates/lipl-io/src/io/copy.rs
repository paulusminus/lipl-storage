use std::path::{Path};
use std::time::{Instant};
use crate::model::{Db, Persist};

pub fn copy<P>(source: P, target: P) -> crate::Result<()> 
where P: AsRef<Path>
{
    let start = Instant::now();

    let mut db = Db::new(source.as_ref().into());
    db.load()?;

    db.save_to(target.as_ref())?;

    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}
