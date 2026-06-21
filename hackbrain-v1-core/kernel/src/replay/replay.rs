use crate::wal::types::{WALRecord, WALEvent};
use serde_json::Value;

#[derive(Default, Debug, Clone)]
pub struct State {
    pub step: u64,
    pub variables: serde_json::Map<String, Value>,
}

/// Replay Engine
/// Theorem 2: Replay(WAL) = S_n is a pure function.
/// NO IO inside apply. NO randomness. NO time dependency.
pub fn replay(wal: Vec<WALRecord>) -> State {
    let mut state = State::default();

    // Sort by authoritative sequence, just in case input is unordered.
    let mut sorted_wal = wal;
    sorted_wal.sort_by_key(|r| r.sequence);

    for record in sorted_wal.iter() {
        state = apply(state, &record.event);
    }

    state
}

fn apply(mut state: State, event: &WALEvent) -> State {
    match event {
        WALEvent::JobStarted(e) => {
            state.variables.insert("input".to_string(), e.input.clone());
        }
        WALEvent::StepExecuted(e) => {
            state.step += 1;
            state.variables.insert(format!("step_{}_output", state.step), Value::String(e.output_hash.clone()));
        }
        WALEvent::ArtifactCreated(e) => {
            state.variables.insert(format!("artifact_{}", e.artifact_id), Value::String(e.content_hash.clone()));
        }
        WALEvent::CheckpointSaved(e) => {
            state.variables.insert("last_checkpoint".to_string(), Value::String(e.checkpoint_id.clone()));
        }
        WALEvent::Recovery(e) => {
            state.variables.insert("recovered_from".to_string(), Value::String(e.recovered_from_checkpoint.clone()));
        }
        // Purely informational events that don't alter the deterministic execution state
        WALEvent::ValidatorTriggered(_) => {}
        WALEvent::SupervisorSignal(_) => {}
        WALEvent::CrashDetected(_) => {}
        WALEvent::JobCompleted(_) => {}
    }

    state
}
