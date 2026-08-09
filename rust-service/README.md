# rust-service

Standalone Rust microservice that generates student PDF reports by calling the existing Node.js backend API. It never connects to the database.

## Endpoints

- `GET /health`
- `GET /api/v1/students/:id/report` returns a downloadable PDF

## Prerequisites

- Rust toolchain (stable)
- PostgreSQL with the seeded `school_mgmt` database
- Node backend running on `http://localhost:5007`

## Setup

```bash
cd rust-service
cp .env.example .env
cargo run
```

Listens on `http://localhost:8080` by default.

## How it talks to the Node API

The student endpoints sit behind JWT auth and CSRF protection, so the service keeps its own session:

1. On the first report request it logs in with `NODE_API_USERNAME` / `NODE_API_PASSWORD` via `POST /api/v1/auth/login`.
2. The login cookies are marked `Secure`, which a cookie jar drops over plain `http://localhost`, so the `Set-Cookie` headers are parsed directly. Login clears the cookies before setting them, so the last non-empty value wins.
3. Subsequent calls to `GET /api/v1/students/:id` send the JWT cookies plus the matching `x-csrf-token` header.
4. The session is cached in memory. If the backend answers 401, 403 or 400 (an expired token fails CSRF validation first), the service logs in again once and retries.

## Test

The seed creates two students, ids `2` (Ada Lovelace) and `3` (Alan Turing).

```bash
curl -OJ http://localhost:8080/api/v1/students/2/report && file student-2-report.pdf
```

Expected: `student-2-report.pdf: PDF document`.

Error cases:

| Request | Response |
|---|---|
| id that is not a student, e.g. the admin `1` | `404 {"error":"student not found"}` |
| unknown id, e.g. `999` | `404 {"error":"student not found"}` |
| non-numeric id, e.g. `abc` | `400` from the path parser, no upstream call |
| Node backend down | `502 {"error":"request failed: ..."}` |

## Tests

```bash
cargo test
```
