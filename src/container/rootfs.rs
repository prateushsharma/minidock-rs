use anyhow::Result;
use nix::mount::{mount, MsFlags};
use nix::unistd::{chdir, chroot};

pub fn change_root(rootfs: Option<&str>) -> Result<()> {
    if let Some(path) = rootfs {
        chroot(path)?;
        chdir("/")?;
    }

    Ok(())
}

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
