import { useMemo, useState } from 'react';

const API_BASE = import.meta.env.VITE_BAGGINS_API || 'http://localhost:8080';
const COMMANDS = ['validate', 'retry', 'escalate', 'hold', 'submit_draft'];

const headers = {
  'x-role': 'biller',
  'x-actor-id': 'operator-biller',
  'content-type': 'application/json',
};

function formatError(detail) {
  if (!detail) return 'Unknown error';
  if (typeof detail === 'string') return detail;
  if (detail?.error?.message) return detail.error.message;
  return JSON.stringify(detail);
}

function now() {
  return new Date().toISOString();
}

export default function App() {
  const [query, setQuery] = useState('');
  const [owner, setOwner] = useState('');
  const [status, setStatus] = useState('');
  const [results, setResults] = useState([]);
  const [selectedCase, setSelectedCase] = useState(null);
  const [messages, setMessages] = useState([]);
  const [command, setCommand] = useState(COMMANDS[0]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');

  const endpoint = useMemo(() => {
    const q = new URLSearchParams();
    if (query.trim()) q.set('q', query.trim());
    if (owner.trim()) q.set('owner', owner.trim());
    if (status.trim()) q.set('status', status.trim());
    q.set('limit', '20');
    return `${API_BASE}/v1/biller/search?${q.toString()}`;
  }, [query, owner, status]);

  const search = async () => {
    setError('');
    setBusy(true);
    try {
      const response = await fetch(endpoint, { headers });
      if (!response.ok) {
        const detail = await response.json().catch(() => ({}));
        throw new Error(formatError(detail));
      }
      const payload = await response.json();
      setResults(payload.items || []);
      setMessages((current) => [
        ...current,
        {
          id: now(),
          speaker: 'assistant',
          text: `Search returned ${payload.items?.length || 0} case(s).`,
          action: 'search',
        },
      ]);
    } catch (err) {
      setError(err.message || 'Search failed');
    } finally {
      setBusy(false);
    }
  };

  const openCase = async (item) => {
    setError('');
    setBusy(true);
    try {
      const response = await fetch(`${API_BASE}/v1/cases/${encodeURIComponent(item.case_id)}`, {
        headers,
      });
      if (!response.ok) {
        const detail = await response.json().catch(() => ({}));
        throw new Error(formatError(detail));
      }
      const payload = await response.json();
      setSelectedCase(payload.case_data);
      const preview = `${payload.case_data.status || 'unknown'} (${payload.case_type})`;
      setMessages((current) => [
        ...current,
        {
          id: now(),
          speaker: 'assistant',
          text: `Loaded case ${item.case_id}. Current state: ${preview}`,
          action: 'open_case',
        },
      ]);
    } catch (err) {
      setError(err.message || 'Unable to open case');
    } finally {
      setBusy(false);
    }
  };

  const execute = async () => {
    if (!selectedCase?.case_id) return;
    setError('');
    setBusy(true);
    try {
      const request = {
        command,
        confirm: true,
        request_id: `biller-${Date.now()}`,
        idempotency_key: `idem-${selectedCase.case_id}-${command}-${Date.now()}`,
        params: {
          source: 'biller-ui-chat',
          submitted_at: now(),
        },
      };

      const response = await fetch(
        `${API_BASE}/v1/cases/${encodeURIComponent(selectedCase.case_id)}/action`,
        {
          method: 'POST',
          headers,
          body: JSON.stringify(request),
        },
      );
      const detail = await response.json();
      if (!response.ok) {
        throw new Error(formatError(detail));
      }

      const summary = `Command ${detail.command} executed for ${selectedCase.case_id}. ` +
        `Trace ${detail.trace_id}, resulting queue ${detail.resulting_queue_state}.`;
      setMessages((current) => [
        ...current,
        { id: now(), speaker: 'user', text: `run ${command}`, action: 'send' },
        { id: now(), speaker: 'assistant', text: summary, action: 'result' },
      ]);
      await openCase(selectedCase);
    } catch (err) {
      setError(err.message || 'Failed to execute command');
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="page">
      <header className="hero">
        <h1>Baggins Biller Workspace</h1>
        <p>Conversational workload navigation for claim review and queue actions.</p>
      </header>

      <section className="search">
        <input
          placeholder="Patient, MRN, or claim"
          value={query}
          onChange={(event) => setQuery(event.target.value)}
        />
        <input
          placeholder="Owner"
          value={owner}
          onChange={(event) => setOwner(event.target.value)}
        />
        <input
          placeholder="Status"
          value={status}
          onChange={(event) => setStatus(event.target.value)}
        />
        <button disabled={busy} onClick={search}>
          Search queue
        </button>
      </section>

      {error && <p className="error">{error}</p>}

      <section className="split">
        <div className="panel">
          <h2>Patient queue</h2>
          <ul className="list">
            {results.map((item) => (
              <li key={item.case_id} className="list-item">
                <button onClick={() => openCase(item)}>
                  {item.patient_name} — {item.claim_id}
                </button>
                <span>{item.status}</span>
              </li>
            ))}
          </ul>
        </div>

        <div className="panel">
          <h2>Conversation command pane</h2>
          <div className="messages">
            {messages.map((entry) => (
              <div key={entry.id} className={`bubble ${entry.speaker}`}>
                <strong>{entry.speaker.toUpperCase()}</strong> {entry.text}
              </div>
            ))}
          </div>
          <label>
            Quick command
            <select value={command} onChange={(event) => setCommand(event.target.value)}>
              {COMMANDS.map((commandValue) => (
                <option key={commandValue} value={commandValue}>
                  {commandValue}
                </option>
              ))}
            </select>
          </label>
          <button disabled={busy || !selectedCase} onClick={execute}>
            Execute on selected case
          </button>
          {selectedCase && (
            <div className="case-box">
              <strong>Selected:</strong> {selectedCase.patient_name} · {selectedCase.case_id}
              <pre>{JSON.stringify(selectedCase.timeline, null, 2)}</pre>
            </div>
          )}
        </div>
      </section>
    </div>
  );
}
