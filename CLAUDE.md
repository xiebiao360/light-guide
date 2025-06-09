# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Architecture Overview

This is a full-stack web application with:
- **Backend**: Rust server using Axum framework with SQLite database
- **Frontend**: Vue 3 SPA with Vite bundler
- **Deployment**: Static files are embedded in the Rust binary using rust-embed

The Rust server serves both API endpoints and static frontend files. The frontend builds to `../static/` directory which gets embedded into the Rust binary.

## Development Commands

### Backend (Rust)
```bash
# Run the server (from project root)
cargo run

# Build the project
cargo build

# Run tests
cargo test
```

### Frontend (Vue.js)
```bash
# Navigate to web directory first
cd web

# Install dependencies
pnpm install

# Development server with hot reload
pnpm dev

# Build for production (outputs to ../static/)
pnpm build

# Format code
pnpm format

# Preview production build
pnpm preview
```

## Key Architecture Details

### Static File Serving
- Frontend builds to `../static/` directory
- Rust server embeds static files using `rust-embed` with compression
- All routes except `/api/*` serve static files or fallback to index.html for SPA routing
- Static files are cached for 1 year, 404s are not cached

### API Structure
- Health check endpoint: `/api/v1/health`
- API routes are nested under `/api` prefix
- Compression middleware is applied to all responses

### Database
- SQLite database file: `guide.db`
- Database file is created automatically if it doesn't exist
- Connection pool managed by SQLx

### Development Workflow
1. Frontend development: Use `pnpm dev` in web/ directory (proxies API calls to localhost:8080)
2. Backend development: Use `cargo run` in project root (serves on port 3000)
3. Production build: Run `pnpm build` in web/ directory, then `cargo build`