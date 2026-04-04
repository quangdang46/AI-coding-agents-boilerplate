use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerStatus {
    Spawning,
    TrustRequired,
    ReadyForPrompt,
    PromptAccepted,
    Running,
    Blocked,
    Finished,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerFailureKind {
    TrustGate,
    PromptDelivery,
    Protocol,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerFailure {
    pub kind: WorkerFailureKind,
    pub message: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerEventKind {
    Spawning,
    TrustRequired,
    TrustResolved,
    ReadyForPrompt,
    PromptAccepted,
    PromptMisdelivery,
    PromptReplayArmed,
    Running,
    Restarted,
    Finished,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerEvent {
    pub seq: u64,
    pub kind: WorkerEventKind,
    pub status: WorkerStatus,
    pub detail: Option<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Worker {
    pub worker_id: String,
    pub cwd: String,
    pub status: WorkerStatus,
    pub trust_auto_resolve: bool,
    pub trust_gate_cleared: bool,
    pub auto_recover_prompt_misdelivery: bool,
    pub prompt_delivery_attempts: u32,
    pub last_prompt: Option<String>,
    pub replay_prompt: Option<String>,
    pub last_error: Option<WorkerFailure>,
    pub created_at: u64,
    pub updated_at: u64,
    pub events: Vec<WorkerEvent>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkerRegistry {
    inner: Arc<Mutex<WorkerRegistryInner>>,
}

#[derive(Debug, Default)]
struct WorkerRegistryInner {
    workers: HashMap<String, Worker>,
    counter: u64,
}

impl WorkerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(
        &self,
        cwd: &str,
        trusted_roots: &[String],
        auto_recover_prompt_misdelivery: bool,
    ) -> Worker {
        let mut inner = self.inner.lock().expect("worker registry lock poisoned");
        inner.counter += 1;
        let ts = now_secs();
        let worker_id = format!("worker_{:08x}_{}", ts, inner.counter);
        let trust_auto_resolve = trusted_roots.iter().any(|root| cwd.starts_with(root));
        let mut worker = Worker {
            worker_id: worker_id.clone(),
            cwd: cwd.to_owned(),
            status: WorkerStatus::Spawning,
            trust_auto_resolve,
            trust_gate_cleared: false,
            auto_recover_prompt_misdelivery,
            prompt_delivery_attempts: 0,
            last_prompt: None,
            replay_prompt: None,
            last_error: None,
            created_at: ts,
            updated_at: ts,
            events: Vec::new(),
        };
        push_event(
            &mut worker,
            WorkerEventKind::Spawning,
            WorkerStatus::Spawning,
            Some("worker created".to_string()),
        );
        inner.workers.insert(worker_id, worker.clone());
        worker
    }
}

fn push_event(
    worker: &mut Worker,
    kind: WorkerEventKind,
    status: WorkerStatus,
    detail: Option<String>,
) {
    let timestamp = now_secs();
    let seq = worker.events.len() as u64 + 1;
    worker.updated_at = timestamp;
    worker.events.push(WorkerEvent {
        seq,
        kind,
        status,
        detail,
        timestamp,
    });
}
#![allow(dead_code)]
