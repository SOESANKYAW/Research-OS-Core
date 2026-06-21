import React, { useState, useEffect } from 'react';
import TraceViewer from './components/TraceViewer';
import KernelPhaseRenderer from './components/KernelPhaseRenderer';
import ReplayEngine from './components/ReplayEngine';
import HubDirectory from './components/HubDirectory';
import { reconstructTrace } from './lib/trace_reconstruction';

function App() {
  const [graph, setGraph] = useState({ nodes: [], edges: [], components: [] });

  // Active SSE Listener for deterministic state machine tracking
  useEffect(() => {
    const eventSource = new EventSource('http://100.104.49.16:4005/api/events/stream');

    eventSource.onmessage = (e) => {
      const newEvent = JSON.parse(e.data);
      // In production, we reconstruct the trace with actual synced artifacts
      // For now, we simulate reconstruction using the live control event history
      setGraph(prev => ({
        ...prev,
        nodes: [...prev.nodes, { 
          id: newEvent.id, 
          logical_time: newEvent.sequence,
          wall_time: newEvent.timestamp,
          survival_state: newEvent.payload?.status || newEvent.event,
          artifact_hash: newEvent.target,
          job_id: newEvent.payload?.job_id
        }]
      }));
    };

    return () => {
      eventSource.close();
    };
  }, []);

  const handleTriggerExecution = async (mode, duration) => {
    try {
      const response = await fetch('http://100.104.49.16:4005/api/control', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          command: 'RUN_STRESS_TEST',
          payload: { mode, duration_ms: duration, script: 'echo "Executing WAL vs DD Probe"' }
        })
      });
      const data = await response.json();
      alert(`Execution Result: ${data.success ? 'Success' : 'Failed'}\nCheck Node Agent console for details.`);
    } catch (err) {
      alert(`Control Plane Error: Could not reach Node Agent at 100.104.49.16:4001.\n${err.message}`);
    }
  };

  const handleSyncArtifacts = async () => {
    try {
      const response = await fetch('http://100.104.49.16:4005/api/sync', {
        method: 'POST'
      });
      const data = await response.json();
      if (data.success) {
        alert(`Successfully synced ${data.artifacts.length} artifacts from OSPC.`);
        // In full production, we would pass data.artifacts into reconstructTrace() here
      } else {
        alert(`Sync Failed: ${data.error}`);
      }
    } catch (err) {
      alert(`Data Plane Error: Could not reach Node Agent at 100.104.49.16:4001.\n${err.message}`);
    }
  };

  return (
    <div className="dashboard-container">
      <div className="left-column" style={{ display: 'flex', flexDirection: 'column', gap: '24px' }}>
        <h1 style={{ color: 'var(--text-primary)', marginBottom: 0 }}>HackBrain Studio v2</h1>
        <p style={{ color: 'var(--text-secondary)', marginTop: 0 }}>Scientific Observability & Execution Ledger</p>
        
        <HubDirectory />
        
        <ReplayEngine graph={graph} />
      </div>
      
      <div className="right-column" style={{ display: 'flex', flexDirection: 'column', gap: '24px' }}>
        <KernelPhaseRenderer graph={graph} />
        <TraceViewer graph={graph} />
      </div>
    </div>
  );
}

export default App;
