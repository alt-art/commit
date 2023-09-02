use std::env::set_current_dir;

use super::*;
use assert_fs::prelude::*;

#[test]
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
fn get_config_path_test() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    set_current_dir(temp_dir.path())?;
    let config_file = dirs::config_dir()
        .ok_or_else(|| anyhow!("Could not find config directory"))?
        .join("commit/commit.json");
    let config_path = get_config_path();
    assert_eq!(config_file.to_str(), config_path?.to_str());
    Ok(())
}

#[test]
fn get_config_path_content_test() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    let config_file = temp_dir.child("config.json");
    config_file.touch()?;
    let config_path = config_file.path();
    let content = get_config_path_content(config_path)?;
    assert_eq!(content, "");

    let expected = include_str!("../../commit-default.json");
    config_file.write_str(expected)?;
    let content = get_config_path_content(config_path)?;
    assert_eq!(content, expected);
    Ok(())
}

#[test]
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
