use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WALRecord {
    pub version: String,
    pub job_id: String,
    pub run_id: String,
    pub created_at: u64,
    pub sequence: u64,
    pub prev_hash: String,
    pub event_hash: String,
    pub fingerprint: ExecutionFingerprint,
    pub event: WALEvent,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ExecutionFingerprint {
    pub provider: String,
    pub model: String,
    pub model_version: String,
    pub temperature: f32,
    pub top_p: Option<f32>,
    pub reasoning_mode: String,
    pub runtime_version: String,
    pub prompt_hash: String,
    pub environment_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum WALEvent {
    JobStarted(JobStartedEvent),
    StepExecuted(StepExecutedEvent),
    ArtifactCreated(ArtifactCreatedEvent),
    CheckpointSaved(CheckpointSavedEvent),
    ValidatorTriggered(ValidatorTriggeredEvent),
    SupervisorSignal(SupervisorSignalEvent),
    CrashDetected(CrashDetectedEvent),
    Recovery(RecoveryEvent),
    JobCompleted(JobCompletedEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobStartedEvent {
    pub input: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StepExecutedEvent {
    pub step_id: String,
    pub input_hash: String,
    pub output_hash: String,
    pub duration_ms: u64,
    pub resource_usage: ResourceUsage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceUsage {
    pub cpu_ms: u64,
    pub memory_mb: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArtifactCreatedEvent {
    pub artifact_id: String,
    pub artifact_type: String,
    pub content_hash: String,
    pub provenance_step_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckpointSavedEvent {
    pub checkpoint_id: String,
    pub state_hash: String,
    pub artifact_snapshot_hash: String,
    pub dependency_graph_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidatorTriggeredEvent {
    pub validator_id: String,
    pub artifact_hash: String,
    pub result: String, // "PASS" | "WARN" | "FAIL"
    pub metadata_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupervisorSignalEvent {
    pub signal: String, // "HEARTBEAT_MISS" | "RETRY" | "KILL" | "FORK_BLOCKED" | "FINGERPRINT_MISMATCH"
    pub reason_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrashDetectedEvent {
    pub crash_signature: String,
    pub last_known_state_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecoveryEvent {
    pub recovered_from_checkpoint: String,
    pub replay_range: (u64, u64),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobCompletedEvent {
    pub output_hash: String,
    pub summary: String,
}
