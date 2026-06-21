import React from 'react';

export default function ControlPanel({ onTriggerExecution, onSyncArtifacts }) {
  return (
    <div className="panel">
      <h2>Control Plane</h2>
      <p>Dispatch immutable command events to the OSPC Execution Node.</p>
      
      <div style={{ display: 'flex', gap: '16px', marginTop: '16px' }}>
        <button 
          className="btn primary" 
          onClick={() => onTriggerExecution('wal_vs_dd', 2000)}
        >
          Inject Pressure (2000ms)
        </button>
        
        <button 
          className="btn" 
          onClick={onSyncArtifacts}
        >
          Sync Linux Artifacts
        </button>
      </div>
    </div>
  );
}
