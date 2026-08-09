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

Requires PostgreSQL 12+, a stable Rust toolchain, and **Node.js 20 or 22**.
Tested on Node 20 and 22. Node 26 fails at startup: `jsonwebtoken` reaches
`buffer-equal-constant-time` through `jws` and `jwa`, and that package touches
`SlowBuffer`, which recent Node releases removed.

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

`backend/.env.example` defaults to `postgresql://postgres:postgres@localhost:5432/school_mgmt`.
Point `DATABASE_URL` at your own PostgreSQL if that role does not exist locally.

3. Frontend, on http://localhost:5173:

```bash
cd frontend && npm install && cp .env.example .env && npm run dev
```

4. Rust service, on http://localhost:8080:

```bash
cd rust-service && cp .env.example .env && cargo run
```

The first build compiles the dependency tree and takes a couple of minutes.

### Demo accounts

Sign in to the frontend as the admin: `admin@school-admin.com` / `3OU4zn3q6Zh9`.

The seed also creates two students, `ada.lovelace@school.com` (id 2) and
`alan.turing@school.com` (id 3), sharing the same password. They exist mainly
as report data. Signing in as one of them and calling a student endpoint
returns 403, because the seed grants those permissions to the admin role only.

## Generate a PDF report

With all three services running:

```bash
curl -f -o student-2-report.pdf http://localhost:8080/api/v1/students/2/report && file student-2-report.pdf
```

One A4 page: a header, then ID, name, email, phone, gender, date of birth,
class, section, roll, admission date, guardian details and addresses. To read
it back on the command line, `pdftotext student-2-report.pdf -`.

The service has its own tests:

```bash
cd rust-service && cargo test
```

## How auth works

The student endpoints require a JWT cookie pair and a matching `x-csrf-token` header. The Rust service logs in once with the service account from its `.env`, parses the `Set-Cookie` headers itself (the cookies are marked `Secure`, which a normal cookie jar drops over plain `http://localhost`), caches the session in memory, and re-authenticates once if the backend rejects a request.

Details, including the error responses, are in [rust-service/README.md](rust-service/README.md).

## What changed in the Node backend

- Implemented the empty handlers in `backend/src/modules/students/students-controller.js`.
- Scoped "student" by joining `roles` on name instead of the hardcoded `role_id = 3`, so listing, detail, status and update all agree on who is a student.
- Stopped `POST /students` from forwarding a client supplied `userId`. The
  `student_add_update` procedure updates in place when the payload carries one,
  so that request could rewrite an existing account instead of adding a student.
- Applied `checkApiAccess` to the student routes. Every other module router
  already does this and the `access_controls` seed defines permissions for
  these five endpoints, but the middleware was never wired up.
- Returned the date columns as calendar dates, so `dob` and `admissionDate` are
  not shifted by a timezone on the way out.
- Turned the `500` on a missing student into a `404`.
- Added sample students to `seed_db/seed-db.sql`, since the original seed created only the admin account.

The `backend/`, `frontend/` and `seed_db/` READMEs are the original template
documents, left unchanged. This file is the setup guide for the submission.

## Limitations

Deliberate scope choices, not oversights:

- The report endpoint has no authentication of its own, and the service holds
  admin credentials. That is fine for a local exercise but a real deployment
  would forward the caller's session, or give the service its own least
  privilege account, and keep port 8080 off the public network.
- The PDF uses the built-in Helvetica, which is WinAnsi encoded. Characters
  outside Latin-1 are rendered as `?` rather than silently dropped. Embedding a
  Unicode font is the upgrade path.
- The report is a single page and field values are cut at 90 characters.
