use std::path::Path;

use crate::config::{checksum, read_optional_text, read_text, RuntimeConfig};

pub struct PromptState {
    pub prompt_texts: Vec<String>,
    pub context_texts: Vec<String>,
    pub prompt_digest: String,
    pub context_digest: String,
}

pub fn load_prompt_state(root: &Path, config: &RuntimeConfig) -> PromptState {
    let mut prompt_texts = vec![read_text(&root.join("__BRAND_DOC__"))];
    for path in [
        "AGENTS.md",
        "CLAUDE.md",
        "__BRAND_ROOT__/__BRAND_DOC__",
        "__BRAND_ROOT__/instructions.md",
    ] {
        if let Some(text) = read_optional_text(&root.join(path)) {
            prompt_texts.push(text);
        }
    }

    let context_texts = config
        .context_paths
        .iter()
        .filter_map(|path| read_optional_text(&root.join(path)))
        .collect::<Vec<_>>();

    PromptState {
        prompt_digest: checksum(&prompt_texts),
        context_digest: checksum(&context_texts),
        prompt_texts,
        context_texts,
    }
}
