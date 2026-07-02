# IAM Technical Design Document

## Objective
Build a multi-tenant Identity and Access Management (IAM) system that supports authentication, authorization, organizations, RBAC, API keys, and audit logging.

## Functional Requirements Implemented
- Authentication and JWT
- Refresh token rotation
- Session management
- Organizations and memberships
- RBAC and permissions
- API keys
- Audit logs
- User profile management

## High-Level Architecture

```text
Client
  ↓
Axum Router
  ↓
Handlers
  ↓
Services
  ↓
Repositories
  ↓
PostgreSQL
```

## Authentication Flow

```text
Register
↓
Login
↓
Access Token + Refresh Token
↓
Protected APIs
↓
Refresh Access Token
↓
Logout
```

## Authorization Flow

```text
User
↓
Membership
↓
Role
↓
Permissions
↓
Access Decision
```

## Database Schema

users
- id
- email
- password_hash
- name
- created_at

organizations
- id
- name
- slug
- created_at

memberships
- user_id
- organization_id
- created_at

roles
- id
- organization_id
- name
- description

permissions
- id
- name
- description

role_permissions
- role_id
- permission_id

member_roles
- membership_id
- role_id

sessions
- id
- user_id
- refresh_token_hash
- expires_at

api_keys
- id
- organization_id
- name
- key_hash
- created_by
- created_at

audit_logs
- id
- actor_id
- organization_id
- action
- resource_type
- resource_id
- created_at

## Security Measures
- Argon2 password hashing
- Hashed refresh tokens
- Hashed API keys
- JWT authentication middleware
- API key middleware
- Authorization checks in services
- Audit trail

## Testing
- Unit tests for password hashing and JWT
- Integration test for register/login flow

## Design Patterns
- Repository Pattern
- Service Layer Pattern
- Dependency Injection via AppState
- DTO-based request/response handling

## Conclusion
The system resembles a simplified production IAM platform with multi-tenancy, RBAC, session management, and machine-to-machine authentication.
