use std::path::Path;

use crate::config::{checksum, read_text, RuntimeConfig};

pub struct PromptState {
    pub prompt_texts: Vec<String>,
    pub context_texts: Vec<String>,
    pub prompt_digest: String,
    pub context_digest: String,
}

pub fn load_prompt_state(root: &Path, config: &RuntimeConfig) -> PromptState {
    let prompt_texts = vec![
        read_text(&root.join("CLAW.md")),
        read_text(&root.join(".agent/prompts/system.md")),
        read_text(&root.join(".agent/prompts/sections/style.md")),
        read_text(&root.join(".agent/prompts/sections/verification.md")),
    ];

    let context_texts = config
        .context_paths
        .iter()
        .map(|path| read_text(&root.join(path)))
        .collect::<Vec<_>>();

    PromptState {
        prompt_digest: checksum(&prompt_texts),
        context_digest: checksum(&context_texts),
        prompt_texts,
        context_texts,
    }
}
