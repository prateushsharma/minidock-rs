use anyhow::Result;
use nix::mount::{mount, MsFlags};

pub fn mount_proc() -> Result<()> {
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::empty(),
        None::<&str>,
    )?;

    Ok(())
}
