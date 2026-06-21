import React, { useState, useEffect } from 'react';
import { inferPhase } from '../lib/phase_inference';

export default function ReplayEngine({ graph }) {
  const [currentTime, setCurrentTime] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const [phase, setPhase] = useState(null);

  // Derive bounds from deterministic partial order
  const minTime = graph.nodes.length > 0 ? Math.min(...graph.nodes.map(n => n.wall_time)) : 0;
  const maxTime = graph.nodes.length > 0 ? Math.max(...graph.nodes.map(n => n.wall_time)) : 100;

  // Sync current time when the graph loads
  useEffect(() => {
    if (minTime > 0) {
      setCurrentTime(minTime);
    }
  }, [minTime]);

  useEffect(() => {
    let interval;
    if (isPlaying) {
      interval = setInterval(() => {
        setCurrentTime(t => {
          if (t >= maxTime) {
            setIsPlaying(false);
            return maxTime;
          }
          return t + 10; // 10ms frame step
        });
      }, 50);
    }
    return () => clearInterval(interval);
  }, [isPlaying, maxTime]);

  useEffect(() => {
    // Pure function projection
    const p = inferPhase(graph, minTime, currentTime);
    setPhase(p);
  }, [currentTime, graph, minTime]);

  return (
    <div className="replay-engine panel">
      <h2>Deterministic Replay Engine</h2>
      <div className="timeline-controls">
        <button className="btn primary" onClick={() => setIsPlaying(!isPlaying)}>
          {isPlaying ? "Pause" : "Play Phase Space"}
        </button>
        <input 
          type="range" 
          min={minTime} 
          max={maxTime} 
          value={currentTime} 
          onChange={(e) => setCurrentTime(Number(e.target.value))}
          className="slider"
        />
        <span className="timestamp">T: {currentTime - minTime}ms</span>
      </div>
      
      {phase && (
        <div className={`phase-banner ${phase.regime.toLowerCase()}`}>
          <h3>Current Phase: {phase.regime}</h3>
          <p>Confidence: {(phase.confidence * 100).toFixed(1)}%</p>
          <p>{phase.description}</p>
        </div>
      )}
    </div>
  );
}
