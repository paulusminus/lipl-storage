use crate::param::{ListCommand, CopyCommand};
use lipl_io::io::{copy as db_copy, list as db_list};
use anyhow::Result;
use log::{info};

pub fn list(args: ListCommand) -> Result<()> {
    db_list(args.source)?;
    Ok(())
}

pub fn copy(args: CopyCommand) -> Result<()> {
    info!(
        "Start copying {} to {}",
        &args.source.to_string_lossy(),
        &args.target.to_string_lossy(),
     );

     db_copy(&args.source, &args.target)?;

     info!(
        "Finished copying {} to {}",
        args.source.to_string_lossy(),
        args.target.to_string_lossy(),
    );
    Ok(())
}