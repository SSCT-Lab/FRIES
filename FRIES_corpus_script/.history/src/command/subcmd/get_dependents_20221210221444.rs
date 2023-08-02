use std::path::PathBuf;

use crates_io_api::{Error, SyncClient};
use run_shell::cmd;

use anyhow::{Ok, Result};
use structopt::StructOpt;

use crate::command::RunCommand;

#[derive(StructOpt)]
pub struct GetDependents {}

impl RunCommand for GetDependents {
    fn run_command(&mut self) -> Result<()> {
        clone_dependents_repositories();
        Ok(())
    }
}
use std::path::{Path, PathBuf};

use crates_io_api::{Error, SyncClient};
use run_shell::cmd;

/// 为某个crate从crates.io上爬下dependents
/// 参数：1.名字 2.依赖最大数目
/// 返回值：String向量
fn get_crate_dependents_repositories(
    name: &str,
    max_dependent_num: i32,
) -> Result<Vec<String>, Error> {
    println!(
        "-----------------Begin to clone dependents of the crate [{}]---------------",
        name
    );
    println!("I will get at most [{}] dependents.", max_dependent_num);

    let mut repos = Vec::new();

    // Instantiate the client.
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(200),
    )
    .unwrap();

    let mut page_idx: u64 = 0; //页号
    let mut dependents_num = 0; //已获取的dependents的数量
    loop {
        //page_idx从1开始，每轮递增，一页一页去拷贝
        page_idx += 1;

        let dependents = match client.crate_reverse_dependencies_page(name, page_idx) {
            Ok(dep) => {
                println!("\x1b[92mPulling down the page {}\x1b[0m", page_idx);
                dep
            }
            Err(_) => {
                println!(
                    "\x1b[91mPage {} doesn't exist\nEnding cloning\x1b[0m",
                    page_idx
                );
                //在dependents页面被爬光后，直接返回
                return Ok(repos);
            }
        };

        for dependent in dependents.dependencies {
            //每次遍历都会加1
            dependents_num += 1;
            let dependent_name = dependent.crate_version.crate_name;

            let dependent_crate_reponse = client.get_crate(&dependent_name)?;
            let dependent_crate = dependent_crate_reponse.crate_data;
            let dependent_repository_addr = match dependent_crate.repository {
                Some(repo) => {
                    repos.push(repo.clone());
                    repo
                }
                None => {
                    continue;
                }
            };
            println!(
                "Find [{}'s] dependent [{}], The repository is [{}].",
                dependent.dependency.crate_id, dependent_name, dependent_repository_addr
            );

            // 在dependents达到数量时直接返回
            if dependents_num >= max_dependent_num {
                return Ok(repos);
            }
        }
    }
}

/// 执行git clone https://github.com/xxx /Users/yxz/dependency/dep/dependents/dependentxxx
fn clone_repository(lib_name: &str, repo_addr: &str, dependents_dir_path: PathBuf, num: u32) {
    //创建目标文件夹，比如/Users/yxz/dependency/dep/dependents/dependent0
    let target_dir =
        dependents_dir_path.join(lib_name.to_string() + "_dependent" + &num.to_string());
    let target_dir = target_dir.to_str().unwrap();

    //如果文件夹存在，先删除
    if Path::new(target_dir).exists() {
        //remove_dir_all(target_dir).unwrap();
        println!(
            "\x1b[92m{}'s dependent {} has exists, we don't have to clone it.\x1b[0m",
            lib_name, num
        );
        return;
    }
    //执行git clone
    let cmd = "git clone ".to_string() + repo_addr + " " + target_dir;
    println!("\x1b[93m{}\x1b[0m", cmd);
    match cmd!(&cmd).run() {
        Ok(_) => todo!(),
        Err(_) => {
            println!("\x1b[91mFailed to clone {}\x1b[0m", repo_addr);
        }
    }
}

/// 对每个dependent仓库都执行git clone，下载到特定的文件夹
fn clone_dependents_repositories(lib_name: &str, max_dependent_num: i32) {
    let crate_root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dependents_dir_path = crate_root_path.join("dependents");
    let repos = get_crate_dependents_repositories(lib_name, max_dependent_num).unwrap();

    let mut dep_cnt = 0;
    for repo in repos {
        clone_repository(lib_name, &repo, dependents_dir_path.clone(), dep_cnt);
        dep_cnt += 1;
    }
}
