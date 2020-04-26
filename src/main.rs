pub mod byte_utils;
pub mod formats;
pub mod common;

use clap::Clap;
use crate::common::R;

#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K.")]
struct RootCmd {

}


fn main() -> R<()> {
    let cmd = RootCmd::parse();

    Ok(())
}
