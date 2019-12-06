use std::{
    env,
    path::PathBuf,
    time::Instant,
};
use failure::ResultExt;

/// Resources that get reused during the lifetime of the program.
pub struct State {
    /// The time at which the program started. Used for telling how much time
    /// has elapsed.
    pub start_time: Instant,
    /// The directory where this process was started from.
    pub current_dir: PathBuf,
    /// The user's home directory.
    pub home_dir: PathBuf,
}

impl State {
    /// Creates a new instance.
    pub fn new() -> crate::Result<Self> {
        let start_time = Instant::now();
        let current_dir = env::current_dir()
            .context("Could not get current working directory")?;
        let home_dir = dirs::home_dir()
            .ok_or(failure::err_msg("Could not get user home directory"))?;
        Ok(Self {
            start_time,
            current_dir,
            home_dir,
        })
    }
}
