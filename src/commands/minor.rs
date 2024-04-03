use crate::versioning::up_minor;
use crate::workspace::{make_context, Workspace};
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub(crate) struct Arguments {}

pub(crate) fn execute(_args: &Arguments, workspace: &mut Workspace) -> Result<()> {
    let current_version = &workspace.config.current_version;
    let new_version = up_minor(&current_version);
    let context = make_context(&current_version, &new_version);
    workspace.update_files(&context)
}
