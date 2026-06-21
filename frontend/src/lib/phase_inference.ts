import { CausalGraph, TraceEvent } from './event_model';

export type PhaseRegime = "D1_STABLE" | "D2_BIFURCATION" | "D3_COLLAPSE";

export interface PhaseProjection {
  regime: PhaseRegime;
  confidence: number;
  description: string;
}

export function inferPhase(graph: CausalGraph, timeWindowStart: number, timeWindowEnd: number): PhaseProjection {
  const eventsInWindow = graph.nodes.filter(e => e.wall_time >= timeWindowStart && e.wall_time <= timeWindowEnd);
  
  if (eventsInWindow.length === 0) {
    return { regime: "D1_STABLE", confidence: 1.0, description: "No events in window." };
  }

  let committedCount = 0;
  let persistedCount = 0;
  let lostCount = 0;

  eventsInWindow.forEach(e => {
    if (e.write_state === "committed") committedCount++;
    if (e.survival_state === "persisted") persistedCount++;
    if (e.survival_state === "lost") lostCount++;
  });

  const survivalRate = committedCount > 0 ? persistedCount / committedCount : 1.0;

  if (survivalRate === 1.0) {
    // Monotonic increase in flushed events, zero survival variance
    return { 
      regime: "D1_STABLE", 
      confidence: 0.95, 
      description: "Stable Writeback: Monotonic flush, low eviction entropy." 
    };
  } else if (survivalRate > 0.0 && survivalRate < 1.0) {
    // Divergence between committed vs persisted
    return { 
      regime: "D2_BIFURCATION", 
      confidence: 0.85, 
      description: "Bifurcation: Nonlinear truncation onset, increasing variance in tail survival." 
    };
  } else {
    // survivalRate === 0
    return { 
      regime: "D3_COLLAPSE", 
      confidence: 0.90, 
      description: "Collapse: Discontinuity in causal graph connectivity, tail cluster annihilation." 
    };
  }
}
