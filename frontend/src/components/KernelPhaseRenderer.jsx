import React from 'react';

export default function KernelPhaseRenderer({ graph }) {
  // A simplistic SVG renderer for the Kernel Phase Field
  // X-axis: logical time
  // Y-axis: durability confidence (0 to 1)
  
  const width = 600;
  const height = 200;
  const padding = 20;
  
  if (graph.nodes.length === 0) return <div className="panel">No data</div>;

  const maxTime = Math.max(...graph.nodes.map(n => n.logical_time));
  
  // Calculate rolling durability to form the Y curve
  const points = [];
  let persistedCount = 0;
  
  graph.nodes.forEach((node, idx) => {
    if (node.survival_state === "persisted") persistedCount++;
    const confidence = persistedCount / (idx + 1);
    
    const x = padding + (node.logical_time / maxTime) * (width - padding * 2);
    const y = height - padding - (confidence * (height - padding * 2));
    points.push(`${x},${y}`);
  });

  return (
    <div className="panel">
      <h2>Kernel Phase Field</h2>
      <svg width="100%" viewBox={`0 0 ${width} ${height}`}>
        <defs>
          <linearGradient id="phaseGrad" x1="0" y1="0" x2="1" y2="0">
            <stop offset="30%" stopColor="var(--success)" />
            <stop offset="60%" stopColor="var(--warning)" />
            <stop offset="90%" stopColor="var(--danger)" />
          </linearGradient>
        </defs>
        <polyline 
          fill="none" 
          stroke="url(#phaseGrad)" 
          strokeWidth="4" 
          points={points.join(" ")} 
        />
        {/* D2 Bifurcation indicator region */}
        <rect x="180" y={padding} width="200" height={height - padding*2} fill="var(--warning)" opacity="0.1" />
        <text x="200" y={height - 5} fill="var(--warning)" fontSize="12">D2 Bifurcation</text>
        
        {/* D3 Collapse indicator region */}
        <rect x="380" y={padding} width="200" height={height - padding*2} fill="var(--danger)" opacity="0.1" />
        <text x="400" y={height - 5} fill="var(--danger)" fontSize="12">D3 Collapse</text>
      </svg>
    </div>
  );
}
