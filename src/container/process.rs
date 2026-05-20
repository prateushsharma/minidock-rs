use crate::cli::RunArgs;
use crate::container::{namespaces, rootfs};
use anyhow::{bail, Result};
use std::process::Command;

pub fn run(args: RunArgs) -> Result<()> {
    if args.command.is_empty() {
        bail!("no command provided");
    }

    namespaces::setup_uts_namespace(args.hostname.as_deref())?;
    namespaces::setup_pid_namespace(args.pid)?;
    namespaces::setup_mount_namespace(args.mount_proc || args.rootfs.is_some())?;

    rootfs::change_root(args.rootfs.as_deref())?;

    if args.mount_proc {
        rootfs::mount_proc()?;
    }

    let program = &args.command[0];
    let program_args = &args.command[1..];

    let status = Command::new(program).args(program_args).status()?;

    if let Some(code) = status.code() {
        std::process::exit(code);
    }

    Ok(())
}
