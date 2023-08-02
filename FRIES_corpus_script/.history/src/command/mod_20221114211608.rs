/*
    This module is used to deal with command line instruction passed by the user.
*/

mod subcmd;

use ::anyhow::Result;
use structopt::StructOpt;

//统一接口，不同命令都统一使用run_command
trait RunCommand {
    fn run_command(&mut self) -> Result<()>;
}

#[derive(StructOpt)]
#[structopt(about = "This is a fuzzing test tool for Rust Library.")]
enum Command {
    Init(subcmd::Init),

    Build(subcmd::Build),
    RUn(subcmd::Run),
}

impl RunCommand for Command {
    fn run_command(&mut self) -> Result<()> {
        match self {
            Command::Init(x) => x.run_command(),
        }
    }
}

pub fn run_command() {
    Command::from_args().run_command();
}
