use std::path::PathBuf;

use crate::error::Result;

pub fn solvertools_dir() -> Result<PathBuf> {
    dotenvy::dotenv().ok();
    Ok(dotenvy::var("SOLVERTOOLS_DIR")?.into())
}
