import { TraceEvent, CausalGraph } from './event_model';

export function buildCausalGraph(events: TraceEvent[]): CausalGraph {
  // Enforce deterministic sorting key: (logical_time, parent_id, wall_time, id)
  const sortedEvents = [...events].sort((a, b) => {
    if (a.logical_time !== b.logical_time) return a.logical_time - b.logical_time;
    if (a.parent_id && b.parent_id && a.parent_id !== b.parent_id) return a.parent_id.localeCompare(b.parent_id);
    if (a.wall_time !== b.wall_time) return a.wall_time - b.wall_time;
    return a.id.localeCompare(b.id);
  });

  const nodes = sortedEvents;
  const edges: { from: string; to: string }[] = [];
  
  // Build edges based on parent_id to enforce partial ordering
  sortedEvents.forEach(event => {
    if (event.parent_id) {
      edges.push({ from: event.parent_id, to: event.id });
    }
  });

  // Simplified connected components (in a real scenario, this would use Tarjan's or Kosaraju's)
  // For WAL physics, contiguous surviving prefixes form the main component
  const components: TraceEvent[][] = [sortedEvents];

  return { nodes, edges, components };
}
