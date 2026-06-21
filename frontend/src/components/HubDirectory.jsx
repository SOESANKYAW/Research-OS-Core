import React, { useState, useEffect } from 'react';

const HubDirectory = () => {
  const [registry, setRegistry] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    // In Vite, proxy handles /api routing to backend port
    fetch('http://100.104.49.16:4001/api/services')
      .then(res => res.json())
      .then(data => {
        if (data.status === 'online') {
          setRegistry(data.registry);
        }
      })
      .catch(err => {
        setError('Control Agent Offline');
      });
  }, []);

  return (
    <div className="panel" style={{ padding: '16px', background: 'var(--panel-bg)', borderRadius: '8px', border: '1px solid var(--border-color)', marginBottom: '24px' }}>
      <h3 style={{ marginTop: 0, color: 'var(--text-primary)', borderBottom: '1px solid var(--border-color)', paddingBottom: '8px' }}>Localhost Topology</h3>
      {error ? (
        <div style={{ color: 'var(--alert-red)' }}>{error}</div>
      ) : registry ? (
        <ul style={{ listStyle: 'none', padding: 0, margin: 0, display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <li>
            <strong style={{ color: 'var(--accent-blue)' }}>Port {registry.control_agent_port}:</strong> Control Agent Backend
          </li>
          <li>
            <strong style={{ color: 'var(--accent-blue)' }}>Port {registry.studio_v2_port}:</strong> HackBrain Studio v2 (Observability)
          </li>
          <li>
            <strong style={{ color: 'var(--accent-blue)' }}>Port {registry.studio_v1_port}:</strong> HackBrain Studio v1 (Bio Data Server)
          </li>
          <li>
            <strong style={{ color: 'var(--accent-blue)' }}>Port {registry.hackbrain_hub_port}:</strong> HackBrain Hub (Obsidian Vault)
          </li>
        </ul>
      ) : (
        <div style={{ color: 'var(--text-secondary)' }}>Polling Registry...</div>
      )}
    </div>
  );
};

export default HubDirectory;
