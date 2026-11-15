import fs from 'node:fs';
import path from 'node:path';

export function createStore(dataDir) {
  fs.mkdirSync(dataDir, { recursive: true });
  const connectionPath = path.join(dataDir, 'connection.json');
  const eventsPath = path.join(dataDir, 'webhook-events.jsonl');
  const sessionsPath = path.join(dataDir, 'es-sessions.jsonl');

  function readConnection() {
    if (!fs.existsSync(connectionPath)) return null;
    return JSON.parse(fs.readFileSync(connectionPath, 'utf8'));
  }

  function writeConnection(patch) {
    const prev = readConnection() || {};
    const next = {
      ...prev,
      ...patch,
      updatedAt: new Date().toISOString(),
    };
    fs.writeFileSync(connectionPath, JSON.stringify(next, null, 2));
    return next;
  }

  function appendJsonl(filePath, obj) {
    fs.appendFileSync(filePath, `${JSON.stringify(obj)}\n`);
  }

  return {
    readConnection,
    writeConnection,
    logSession(event) {
      appendJsonl(sessionsPath, { at: new Date().toISOString(), ...event });
    },
    logWebhook(event) {
      appendJsonl(eventsPath, { at: new Date().toISOString(), ...event });
    },
    recentWebhooks(limit = 20) {
      if (!fs.existsSync(eventsPath)) return [];
      const lines = fs.readFileSync(eventsPath, 'utf8').trim().split('\n').filter(Boolean);
      return lines.slice(-limit).map((line) => JSON.parse(line));
    },
  };
}
