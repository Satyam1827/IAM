# IAM (Identity and Access Management) Service

## Overview
IAM is a backend service built in Rust that provides authentication, authorization, organization management, role-based access control (RBAC), session management, API keys, and audit logging.

## Tech Stack
- Rust
- Axum
- Tokio
- PostgreSQL
- SQLx
- JWT
- Argon2
- UUID
- Serde

## Features

### Authentication
- User registration
- User login
- Password hashing using Argon2
- JWT access tokens
- Refresh token rotation
- Logout
- Session management

### Organizations
- Create organizations
- List organization members
- Add members
- Remove members

### RBAC
- Create roles
- Assign permissions to roles
- Assign roles to members
- Permission checks

### API Keys
- Create API keys
- List API keys
- Revoke API keys
- API-key authentication middleware

### User Profile
- Get current user profile
- Update profile name

### Audit Logs
Tracks:
- Organization creation
- Member management
- Role creation
- Role assignment
- Permission assignment

## Project Structure

```text
src/
├── auth/
├── config/
├── db/
│   ├── models/
│   └── repositories/
├── dto/
├── handlers/
├── middleware/
├── routes/
├── services/
├── state.rs
├── app.rs
└── main.rs
```

## Architecture

```text
Request
↓
Route
↓
Handler
↓
Service
↓
Repository
↓
PostgreSQL
```

## Database Tables
- users
- organizations
- memberships
- roles
- permissions
- role_permissions
- member_roles
- sessions
- api_keys
- audit_logs

## Main Endpoints

### Authentication
- POST /auth/register
- POST /auth/login
- POST /auth/refresh
- POST /auth/logout
- GET /auth/sessions
- DELETE /auth/sessions/{id}

### User
- GET /users/me
- PATCH /users/me

### Organizations
- POST /organizations
- GET /organizations/{id}/members
- POST /organizations/{id}/members
- DELETE /organizations/{id}/members/{user_id}

### Roles
- POST /organizations/{id}/roles
- POST /organizations/{id}/roles/{role_id}/permissions
- POST /organizations/{id}/members/{user_id}/roles

### API Keys
- POST /organizations/{id}/api-keys
- GET /organizations/{id}/api-keys
- DELETE /organizations/{id}/api-keys/{key_id}

## Running Locally

```bash
cargo run
```

## Running Tests

```bash
cargo test
```

## Future Improvements
- Pagination and filtering
- Email verification
- MFA
- OAuth login
