use crate::commit_message::message_build::MessageBuilder;
use crate::config::Config;
use anyhow::Result;

fn message_with_config(config: Config) -> String {
    let mut builder = MessageBuilder::new(config);
    builder.set_type("feat");
    builder.set_scope("test");
    builder.set_description("description");
    builder.set_body("body");
    builder.set_footer("footer");
    builder.message
}

#[test]
fn message_builder_config_test() -> Result<()> {
    let mut config = Config {
        scope_prefix: "(".to_string(),
        scope_suffix: ")".to_string(),
        subject_separator: ": ".to_string(),
        type_prefix: None,
        type_suffix: None,
    };
    assert_eq!(
        message_with_config(config.clone()),
        "feat(test): description\n\nbody\n\nfooter"
    );
    config.type_prefix = Some("[".to_string());
    config.type_suffix = Some("]".to_string());
    assert_eq!(
        message_with_config(config.clone()),
        "[feat](test): description\n\nbody\n\nfooter"
    );
    config.subject_separator = " ".to_string();
    assert_eq!(
        message_with_config(config.clone()),
        "[feat](test) description\n\nbody\n\nfooter"
    );
    config.scope_prefix = "".to_string();
    config.scope_suffix = "".to_string();
    assert_eq!(
        message_with_config(config.clone()),
        "[feat]test description\n\nbody\n\nfooter"
    );
    Ok(())
}

#[test]
fn message_builder_test() -> Result<()> {
    let config = Config {
        scope_prefix: "(".to_string(),
        scope_suffix: ")".to_string(),
        subject_separator: ": ".to_string(),
        type_prefix: None,
        type_suffix: None,
    };
    let mut builder = MessageBuilder::new(config);
    builder.set_type("feat");
    assert_eq!(builder.message, "feat");
    builder.set_scope("test");
    assert_eq!(builder.message, "feat(test)");
    builder.set_description("description");
    assert_eq!(builder.message, "feat(test): description");
    builder.set_body("body");
    assert_eq!(builder.message, "feat(test): description\n\nbody");
    builder.set_footer("footer");
    assert_eq!(builder.message, "feat(test): description\n\nbody\n\nfooter");
    Ok(())
}
