# Repository Guidelines

## Project Structure & Module Organization
- `frontend/`: React + TypeScript app (Vite). Source in `src/`, assets in `public/`, optional WASM helpers in `wasm/` (Rust targeting WebAssembly).
- `nginx/`: Containerized reverse proxy forwarding to the Vite dev server.
- `compose.yaml`: Local orchestration for `nginx` and `frontend`.
- Root `Dockerfile`: Base image helpers for the stack (see service-level Dockerfiles as primary).

## Build, Test, and Development Commands
- Local dev (no Docker): `cd frontend && npm install && npm run dev` → http://localhost:5173
- Dev with Docker Compose: `docker compose up --build` → http://localhost:8080 via Nginx.
- Lint: `cd frontend && npm run lint` (TypeScript + React rules).
- Production build: `cd frontend && npm run build`; preview locally via `npm run preview`.

## Coding Style & Naming Conventions
- TypeScript, 2‑space indentation; prefer explicit types at module boundaries.
- React components: PascalCase filenames (e.g., `GridCanvas.tsx`); hooks: `use*.ts`.
- Utilities: `.ts` under `src/`; keep imports relative and tidy.
- ESLint is configured (`eslint.config.js`) with TypeScript + React Hooks. Fix lint before pushing.

## Testing Guidelines
- No formal test runner is configured yet. If adding tests, prefer Vitest + React Testing Library.
- Naming: place beside source as `*.test.ts`/`*.test.tsx`.
- Aim for >80% coverage on new/changed code and include meaningful edge cases.

## Commit & Pull Request Guidelines
- Follow Conventional Commits: `feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, `test:` (seen in history).
- PRs must include: clear description, linked issue (if any), screenshots/GIFs for UI, and run steps.
- Ensure: build passes, lint clean, and manual sanity check in both `5173` (dev) and `8080` (via Nginx) when relevant.

## Security & Configuration Tips
- Do not commit secrets. Prefer environment variables and `.env` (excluded via `.gitignore`).
- Nginx proxies HMR/WebSocket; when changing dev hosts/ports, update `vite.config.ts` and Nginx conf together.
