/*
    This is core module to implement the core functionility of the fuzzing target
*/
mod util;

use anyhow::Result;
use std::path::PathBuf;

use crate::command::subcmd::Init;

use self::util::find_this_package;

const DEFAULT_FUZZ_DIR_NAME: &str = "fuzz";
const DEFAULT_FUZZ_TARGET_DIR_NAME: &str = "target";

pub struct FuzzProject {
    project_dir: PathBuf,     //被测试项目的主目录
    fuzz_dir: PathBuf,        //测试目录，装有测试程序，默认是 {project_dir}/fuzz
    target_dir: PathBuf,      //测试目标所在的目录，默认是 {fuzz_dir}/target_dir
    target_name: Vec<String>, //测试目标，可以后续添加......
}

impl FuzzProject {
    fn init(init: &mut Init) -> Result<Self> {
        // 获取当前项目的主目录
        let this_package_path = find_this_package()?;
        // 获取或生成fuzz test所在的目录
        let fuzz_dir = match init.fuzz_dir_wrapper.fuzz_dir {
            Some(dir) => dir, //如果用户提供了，则使用用户的目录
            None => this_package_path.join(DEFAULT_FUZZ_DIR_NAME), //否则，使用默认目录
        };

        let target_dir = match init.target_name {
            Some(dir) => PathBuf::from(dir),
            None => fuzz_dir.join(DEFAULT_FUZZ_TARGET_DIR_NAME),
        };

        Ok(FuzzProject {
            fuzz_dir,
            target_dir: (),
            target_name: (),
        })
    }
}
