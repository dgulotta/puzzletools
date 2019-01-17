use std::path::PathBuf;

use error::Result;

pub fn solvertools_dir() -> Result<PathBuf> {
    dotenv::dotenv().ok();
    Ok(dotenv::var("SOLVERTOOLS_DIR")?.into())
}
