# School Management System with a Rust PDF report service

The original stack (React frontend, Express backend, PostgreSQL) plus `rust-service/`, a standalone Rust microservice that generates student PDF reports by consuming the Node API.

## Architecture

```
frontend/ (React + Vite, :5173)
      |
      v
backend/ (Express + PostgreSQL, :5007)  <----  rust-service/ (Axum, :8080)
      |                                        fetches student JSON over HTTP,
      v                                        renders the PDF, no DB access
 PostgreSQL (school_mgmt)
```

`rust-service` has no database credentials and no database driver. It authenticates against the Node API like any other client and reads `GET /api/v1/students/:id`.

## Quick start

Requires PostgreSQL 12+ and **Node.js 20 or 22**. Node 26 breaks the backend's JWT dependency chain (`buffer-equal-constant-time`), so use an LTS release.

Run each step from the repository root. Steps 2 to 4 are long running, so give each one its own terminal.

1. Database:

```bash
createdb school_mgmt && psql -d school_mgmt -f seed_db/tables.sql && psql -d school_mgmt -f seed_db/seed-db.sql
```

Run the seed once, against a fresh database. Applying it twice duplicates the
`access_controls` rows, which shows up as repeated menu entries in the sidebar.

2. Backend, on http://localhost:5007:

```bash
cd backend && npm install && cp .env.example .env && npm start
```

3. Frontend, on http://localhost:5173:

```bash
cd frontend && npm install && cp .env.example .env && npm run dev
```

4. Rust service, on http://localhost:8080:

```bash
cd rust-service && cp .env.example .env && cargo run
```

Point `DATABASE_URL` in `backend/.env` at your own PostgreSQL if the default differs.

### Demo accounts

| Role | Email | Password |
|---|---|---|
| Admin | `admin@school-admin.com` | `3OU4zn3q6Zh9` |
| Student (id 2) | `ada.lovelace@school.com` | `3OU4zn3q6Zh9` |
| Student (id 3) | `alan.turing@school.com` | `3OU4zn3q6Zh9` |

## Generate a PDF report

With all three services running:

```bash
curl -OJ http://localhost:8080/api/v1/students/2/report && file student-2-report.pdf
```

## How auth works

The student endpoints require a JWT cookie pair and a matching `x-csrf-token` header. The Rust service logs in once with the service account from its `.env`, parses the `Set-Cookie` headers itself (the cookies are marked `Secure`, which a normal cookie jar drops over plain `http://localhost`), caches the session in memory, and re-authenticates once if the backend rejects a request.

Details, including the error responses, are in [rust-service/README.md](rust-service/README.md).

## What changed in the Node backend

- Implemented the empty handlers in `backend/src/modules/students/students-controller.js`.
- Scoped "student" by joining `roles` on name instead of the hardcoded `role_id = 3`, so listing, detail, status and update all agree on who is a student.
- Added sample students to `seed_db/seed-db.sql`, since the original seed created only the admin account.
