import React from 'react';

export default function TraceViewer({ graph }) {
  return (
    <div className="panel">
      <h2>Raw Artifact Inspector</h2>
      <div className="trace-viewer">
        {graph.nodes.length === 0 ? (
          <p>No trace artifacts loaded.</p>
        ) : (
          graph.nodes.map(node => (
            <div key={node.id} className={`trace-item ${node.survival_state}`}>
              <span>[{node.logical_time}] {node.job_id || node.id.substring(0,8)}</span>
              <span>{node.survival_state.toUpperCase()}</span>
              <span>Target: {node.artifact_hash}</span>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
