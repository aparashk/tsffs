use std::path::PathBuf;

use anyhow::{bail, Result};
use dotenvy_macro::dotenv;

const SIMICS_HOME: &str = dotenv!("SIMICS_HOME");

/// Return the SIMICS_HOME directory as a PathBuf. This depends on the SIMICS_HOME environment
/// variable being defined at compile time, and runtime changes to this variable will have no
/// effect.
pub fn simics_home() -> Result<PathBuf> {
    let simics_home = PathBuf::from(SIMICS_HOME);
    match simics_home.exists() {
        true => Ok(simics_home),
        false => {
            bail!(
                "SIMICS_HOME is defined, but {} does not exist.",
                SIMICS_HOME
            )
        }
    }
}