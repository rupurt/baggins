# Baggins Interfaces

This repository contains two React frontends used by the conversational UIs in mission `VFTDIU8bi`:

- `interfaces/biller` — Biller-facing search + queue + conversational action panel.
- `interfaces/payer` — Payer-facing denial triage + appeals panel.

Each UI points to the baggins API base at:

- `http://localhost:8080` by default

Override in Vite with:

- `VITE_BAGGINS_API` (environment variable)

Build targets:

- `just build-frontend` builds both apps
- `just serve-biller-ui` starts the biller app at port 5173
- `just serve-payer-ui` starts the payer app at port 5174
