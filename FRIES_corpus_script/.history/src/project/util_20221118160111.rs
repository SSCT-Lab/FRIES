/*
    There are tools for dealing with package finding and path spilting.
*/
use anyhow::{bail, Context, Result};
use std::io::{Read, Write};
use std::{env, fs, path::PathBuf};

pub fn is_fuzz_cargo_toml(value: &mut toml::Value) -> bool {
    false
}

//获取当前工作目录所在project的Cargo.toml的Value
pub fn get_this_project_toml_value() -> Result<toml::Value> {
    let mut dir_path = env::current_dir()?;
    get_project_toml_value(dir_path)
}

//获取某个目录下Cargo.toml的Value
pub fn get_project_toml_value(dir_path: PathBuf) -> Result<toml::Value> {
    let mut data = Vec::new();
    let cargo_toml_path = dir_path.join("Cargo.toml");
    match fs::File::open(&cargo_toml_path) {
        Ok(mut f) => {
            f.read_to_end(&mut data).with_context(|| {
                format!("failed to read {} toml file", cargo_toml_path.display())
            })?; //懒惰求值,
            let value: toml::Value = toml::from_slice(&data).with_context(|| {
                format!("failed to decode the file at {}", cargo_toml_path.display())
            })?;
            Ok(value)
        }
        Err(_) => {
            bail!(
                "We can't find the package according to this working dir:{}",
                env::current_dir().unwrap().display()
            );
        }
    }
}

/// 找到当前工作目录所在的项目的主目录
/// 该目录里应有Cargo.toml等配置文件
pub fn find_this_package() -> Result<PathBuf> {
    let mut dir = env::current_dir()?;
    let mut data = Vec::new();
    let cargo_toml_path = dir.join("Cargo.toml");

    match fs::File::open(&cargo_toml_path) {
        Ok(mut f) => {
            data.clear();

            //读取Cargo.toml内部的值
            f.read_to_end(&mut data).with_context(|| {
                format!("failed to read {} toml file", cargo_toml_path.display())
            })?; //懒惰求值

            let value: toml::Value = toml::from_slice(&data).with_context(|| {
                format!("failed to decode the file at {}", cargo_toml_path.display())
            })?;
            return Ok(dir);
        }
        Err(_) => {
            bail!(
                "We can't find the package according to this working dir:{}",
                env::current_dir().unwrap().display()
            );
        }
    }
}
