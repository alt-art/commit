use std::{env::set_current_dir, fs::remove_file};

use super::*;
use assert_fs::prelude::*;
use serial_test::serial;

#[test]
#[serial]
fn select_custom_config_path_test() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    let config_file = temp_dir.child("config.json");
    config_file.touch()?;

    let config_path = config_file.path().to_owned();
    let selected_config_path = select_custom_config_path(Some(config_path.clone()))?;
    assert_eq!(config_path.to_str(), selected_config_path.to_str());

    set_current_dir(temp_dir.path())?;
    let config_path_default = dirs::config_dir().unwrap().join("commit/commit.json");
    let selected_config_path = select_custom_config_path(None)?;
    assert_eq!(selected_config_path.to_str(), config_path_default.to_str());

    let selected_config_path = select_custom_config_path(Some(PathBuf::new()));
    match selected_config_path {
        Err(err) => assert_eq!(err.to_string(), "Config file does not exist: "),
        _ => unreachable!(),
    }
    Ok(())
}

#[test]
#[serial]
fn get_config_path_test() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    temp_dir.child("commit.json").touch()?;
    temp_dir.child("some/sub/dir").create_dir_all()?;
    set_current_dir(temp_dir.path().join("some/sub/dir"))?;
    let config_path = get_config_path()?;
    assert_eq!(
        config_path.to_str(),
        temp_dir.path().join("commit.json").to_str()
    );
    remove_file(temp_dir.path().join("commit.json"))?;
    let config_file = dirs::config_dir()
        .ok_or_else(|| anyhow!("Could not find config directory"))?
        .join("commit/commit.json");
    let config_path = get_config_path();
    assert_eq!(config_file.to_str(), config_path?.to_str());
    Ok(())
}

#[test]
#[serial]
fn get_pattern_test() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    set_current_dir(temp_dir.path())?;
    let pattern = get_pattern(None)?;
    assert_eq!(pattern.config.type_prefix, None);
    assert_eq!(pattern.config.type_suffix, None);
    assert_eq!(pattern.config.subject_separator, ": ");
    assert_eq!(pattern.config.scope_prefix, "(");
    assert_eq!(pattern.config.scope_suffix, ")");
    Ok(())
}
