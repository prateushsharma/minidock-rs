use anyhow::Result;
use nix::sched::{unshare, CloneFlags};
use nix::unistd::sethostname;

pub fn setup_uts_namespace(hostname: Option<&str>) -> Result<()> {
    if let Some(name) = hostname {
        unshare(CloneFlags::CLONE_NEWUTS)?;
        sethostname(name)?;
    }

    Ok(())
}
