# AGENTS.md — Voyage contributor notes

This file is for coding agents (Codex/Claude/etc.) making changes in this repo.

## API + Types contract

Voyage uses a two-layer API type model:

1) **Transport (generated):** `src/generated/api_types.rs`
- Generated from backend Swagger (`../voyage-backend/docs/swagger.json`)
- Regenerate with: `just generate-api-types`

2) **App-facing models (hand-authored):** `src/api.rs`
- Map generated transport structs into app-safe UI/domain structs
- Keep fallback/default logic here, not in view components

## Required workflow for API changes

When adding/changing API endpoints:
1. Update backend swagger/source first (in `voyage-backend`)
2. Run `just generate-api-types`
3. Update mapping logic in `src/api.rs`
4. Run `just check-api-types` and `cargo check`
5. Include generated file updates in the same PR

## Guardrails

- Do **not** decode backend JSON directly in views.
- Do **not** hand-edit `src/generated/api_types.rs`.
- Keep assumptions about backend schema documented inline in `src/api.rs`.
- Prefer explicit mapping over leaking transport types across app boundaries.
