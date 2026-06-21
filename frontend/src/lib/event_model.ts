export type WriteState = "committed" | "pending" | "flushed";
export type SurvivalState = "persisted" | "lost" | "unknown";

export interface TraceEvent {
  id: string;
  parent_id?: string;
  logical_time: number;
  wall_time: number;
  write_state: WriteState;
  survival_state: SurvivalState;
  artifact_hash: string;
}

export interface CausalGraph {
  nodes: TraceEvent[];
  edges: { from: string; to: string }[];
  components: TraceEvent[][];
}
