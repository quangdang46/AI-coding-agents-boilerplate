#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchFreshness {
    Fresh,
    Stale {
        commits_behind: usize,
        missing_fixes: Vec<String>,
    },
    Diverged {
        ahead: usize,
        behind: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaleBranchPolicy {
    AutoRebase,
    AutoMergeForward,
    WarnOnly,
    Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StaleBranchAction {
    Noop,
    Warn { message: String },
    Block { message: String },
    Rebase,
    MergeForward,
}

pub fn apply_policy(freshness: &BranchFreshness, policy: StaleBranchPolicy) -> StaleBranchAction {
    match freshness {
        BranchFreshness::Fresh => StaleBranchAction::Noop,
        BranchFreshness::Stale { commits_behind, missing_fixes } => match policy {
            StaleBranchPolicy::WarnOnly => StaleBranchAction::Warn {
                message: format!(
                    "Branch is {commits_behind} commit(s) behind main. Missing fixes: {}",
                    if missing_fixes.is_empty() { "(none)".to_string() } else { missing_fixes.join("; ") }
                ),
            },
            StaleBranchPolicy::Block => StaleBranchAction::Block {
                message: format!("Branch is {commits_behind} commit(s) behind main and must be updated before proceeding."),
            },
            StaleBranchPolicy::AutoRebase => StaleBranchAction::Rebase,
            StaleBranchPolicy::AutoMergeForward => StaleBranchAction::MergeForward,
        },
        BranchFreshness::Diverged { ahead, behind } => match policy {
            StaleBranchPolicy::WarnOnly => StaleBranchAction::Warn {
                message: format!("Branch has diverged: {ahead} commit(s) ahead, {behind} commit(s) behind main."),
            },
            StaleBranchPolicy::Block => StaleBranchAction::Block {
                message: format!("Branch has diverged ({ahead} ahead, {behind} behind) and must be reconciled before proceeding."),
            },
            StaleBranchPolicy::AutoRebase => StaleBranchAction::Rebase,
            StaleBranchPolicy::AutoMergeForward => StaleBranchAction::MergeForward,
        },
    }
}
