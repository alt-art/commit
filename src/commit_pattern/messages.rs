use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Messages {
    #[serde(default = "Messages::commit_type")]
    pub commit_type: String,
    #[serde(default = "Messages::commit_scope")]
    pub commit_scope: String,
    #[serde(default = "Messages::commit_description")]
    pub commit_description: String,
    #[serde(default = "Messages::commit_body")]
    pub commit_body: String,
    #[serde(default = "Messages::commit_footer")]
    pub commit_footer: String,
}

impl Messages {
    fn commit_type() -> String {
        "What type of commit you will made?".to_owned()
    }
    fn commit_scope() -> String {
        "What scope of commit you will made? (Optional)".to_owned()
    }
    fn commit_description() -> String {
        "Write a SHORT, IMPERATIVE tense description of the change:".to_owned()
    }
    fn commit_body() -> String {
        "Provide a LONGER description of the change (Optional):".to_owned()
    }

    fn commit_footer() -> String {
        "List any ISSUES CLOSED by this change E.g.: #31, #34 (Optional):".to_owned()
    }
}

impl Default for Messages {
    fn default() -> Self {
        Self {
            commit_type: Self::commit_type(),
            commit_scope: Self::commit_scope(),
            commit_description: Self::commit_description(),
            commit_body: Self::commit_body(),
            commit_footer: Self::commit_footer(),
        }
    }
}
