import { useState } from 'react';

const API_BASE = import.meta.env.VITE_BAGGINS_API || 'http://localhost:8080';
const COMMANDS = ['triage', 'draft_appeal', 'submit_response', 'escalate', 'hold'];

const headers = {
  'x-role': 'payer',
  'x-actor-id': 'operator-payer',
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
  const [payer, setPayer] = useState('');
  const [reason, setReason] = useState('');
  const [status, setStatus] = useState('');
  const [ageMin, setAgeMin] = useState('');
  const [ageMax, setAgeMax] = useState('');
  const [results, setResults] = useState([]);
  const [selectedCase, setSelectedCase] = useState(null);
  const [messages, setMessages] = useState([]);
  const [command, setCommand] = useState(COMMANDS[0]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');

  const doSearch = async () => {
    setBusy(true);
    setError('');
    try {
      const params = new URLSearchParams();
      if (payer.trim()) params.set('payer', payer.trim());
      if (reason.trim()) params.set('reason', reason.trim());
      if (status.trim()) params.set('status', status.trim());
      if (ageMin.trim()) params.set('age_min', ageMin.trim());
      if (ageMax.trim()) params.set('age_max', ageMax.trim());
      params.set('limit', '20');

      const response = await fetch(`${API_BASE}/v1/payer/denials/search?${params.toString()}`, {
        headers,
      });
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
          text: `Found ${payload.items?.length || 0} denial(s).`,
          action: 'search',
        },
      ]);
    } catch (err) {
      setError(err.message || 'Search failed');
    } finally {
      setBusy(false);
    }
  };

  const openCase = async (caseItem) => {
    setBusy(true);
    setError('');
    try {
      const response = await fetch(`${API_BASE}/v1/cases/${encodeURIComponent(caseItem.case_id)}`, {
        headers,
      });
      if (!response.ok) {
        const detail = await response.json().catch(() => ({}));
        throw new Error(formatError(detail));
      }
      const payload = await response.json();
      setSelectedCase(payload.case_data);
      setMessages((current) => [
        ...current,
        {
          id: now(),
          speaker: 'assistant',
          text: `Loaded denial case ${caseItem.case_id} for ${caseItem.patient_name}.`,
          action: 'open_case',
        },
      ]);
    } catch (err) {
      setError(err.message || 'Could not open case');
    } finally {
      setBusy(false);
    }
  };

  const execute = async () => {
    if (!selectedCase?.case_id) return;
    setBusy(true);
    setError('');
    try {
      const response = await fetch(
        `${API_BASE}/v1/cases/${encodeURIComponent(selectedCase.case_id)}/action`,
        {
          method: 'POST',
          headers,
          body: JSON.stringify({
            command,
            confirm: true,
            request_id: `payer-${Date.now()}`,
            idempotency_key: `idem-${selectedCase.case_id}-${command}-${Date.now()}`,
            params: {
              source: 'payer-ui-chat',
              submitted_at: now(),
            },
          }),
        },
      );
      const detail = await response.json();
      if (!response.ok) {
        throw new Error(formatError(detail));
      }
      setMessages((current) => [
        ...current,
        {
          id: now(),
          speaker: 'user',
          text: `Executed ${command} on ${selectedCase.case_id}`,
          action: 'send',
        },
        {
          id: now(),
          speaker: 'assistant',
          text: `Result: ${detail.command_outcome}. Transition ${detail.transition_id}.`,
          action: 'result',
        },
      ]);
      await openCase(selectedCase);
    } catch (err) {
      setError(err.message || 'Action failed');
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="page">
      <header className="hero">
        <h1>Baggins Payer Workspace</h1>
        <p>Denial triage, conversational recommendations, and appeal response actions.</p>
      </header>

      <section className="search">
        <input
          placeholder="Payer"
          value={payer}
          onChange={(event) => setPayer(event.target.value)}
        />
        <input
          placeholder="Denial reason"
          value={reason}
          onChange={(event) => setReason(event.target.value)}
        />
        <input
          placeholder="Status"
          value={status}
          onChange={(event) => setStatus(event.target.value)}
        />
        <input
          placeholder="Age min"
          value={ageMin}
          onChange={(event) => setAgeMin(event.target.value)}
        />
        <input
          placeholder="Age max"
          value={ageMax}
          onChange={(event) => setAgeMax(event.target.value)}
        />
        <button disabled={busy} onClick={doSearch}>
          Search denials
        </button>
      </section>

      {error && <p className="error">{error}</p>}

      <section className="split">
        <div className="panel">
          <h2>Denial queue</h2>
          <ul className="list">
            {results.map((item) => (
              <li key={item.case_id} className="list-item">
                <button onClick={() => openCase(item)}>
                  {item.patient_name} — {item.denial_reason}
                </button>
                <span>{item.age_days}d</span>
              </li>
            ))}
          </ul>
        </div>
        <div className="panel">
          <h2>Conversation panel</h2>
          <div className="messages">
            {messages.map((entry) => (
              <div key={entry.id} className={`bubble ${entry.speaker}`}>
                <strong>{entry.speaker.toUpperCase()}</strong> {entry.text}
              </div>
            ))}
          </div>
          <label>
            Action
            <select value={command} onChange={(event) => setCommand(event.target.value)}>
              {COMMANDS.map((commandValue) => (
                <option key={commandValue} value={commandValue}>
                  {commandValue}
                </option>
              ))}
            </select>
          </label>
          <button disabled={busy || !selectedCase} onClick={execute}>
            Execute workflow command
          </button>
          {selectedCase && (
            <div className="case-box">
              <strong>Selected:</strong> {selectedCase.patient_name} · {selectedCase.case_id}
              <pre>{JSON.stringify(selectedCase.timeline || [], null, 2)}</pre>
            </div>
          )}
        </div>
      </section>
    </div>
  );
}
