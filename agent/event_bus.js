const crypto = require('crypto');
const fs = require('fs');
const path = require('path');

class EventBus {
  constructor(logDir) {
    this.logDir = logDir;
    if (!fs.existsSync(this.logDir)) {
      fs.mkdirSync(this.logDir, { recursive: true });
    }
    this.logFile = path.join(this.logDir, `control_events_${Date.now()}.jsonl`);
    this.events = [];
    this.subscribers = [];
    this.sequence = 0;
  }

  subscribe(res) {
    this.subscribers.push(res);
  }

  unsubscribe(res) {
    this.subscribers = this.subscribers.filter(client => client !== res);
  }

  dispatch(eventName, target, payload) {
    this.sequence++;
    const event = {
      id: crypto.randomUUID(),
      sequence: this.sequence,
      event: eventName,
      target: target,
      payload: payload,
      timestamp: Date.now()
    };
    
    this.events.push(event);
    fs.appendFileSync(this.logFile, JSON.stringify(event) + '\n');
    
    // Broadcast to SSE clients
    const ssePayload = `data: ${JSON.stringify(event)}\n\n`;
    this.subscribers.forEach(client => client.write(ssePayload));
    
    return event;
  }

  getHistory() {
    return this.events;
  }
}

module.exports = EventBus;
