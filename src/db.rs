use crate::param::{ListCommand, CopyCommand};
use lipl_io::io::{copy as db_copy, list as db_list};
use anyhow::Result;

pub fn list(args: ListCommand) -> Result<()> {
    db_list(args.source)?;
    Ok(())
}

pub fn copy(args: CopyCommand) -> Result<()> {
    db_copy(args.source, args.target)?;
    Ok(())
}