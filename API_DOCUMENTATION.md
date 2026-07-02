# API_DOCUMENTATION

Base URL:

```text
http://localhost:3000
```

## Authentication

### Register
```http
POST /auth/register
```

Request:
```json
{
  "email": "user@example.com",
  "password": "password123",
  "name": "John Doe"
}
```

Response:
```http
200 OK
```

---

### Login
```http
POST /auth/login
```

Request:
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

Response:
```json
{
  "access_token": "<jwt>",
  "refresh_token": "<refresh_token>",
  "session_id": "<uuid>"
}
```

---

### Refresh Token
```http
POST /auth/refresh
```

Request:
```json
{
  "session_id": "<uuid>",
  "refresh_token": "<refresh_token>"
}
```

Response:
```json
{
  "access_token": "<jwt>",
  "refresh_token": "<new_refresh_token>",
  "session_id": "<uuid>"
}
```

---

### Logout
```http
POST /auth/logout
Authorization: Bearer <access_token>
```

Response:
```http
200 OK
```

---

### List Sessions
```http
GET /auth/sessions
Authorization: Bearer <access_token>
```

---

### Revoke Session
```http
DELETE /auth/sessions/{session_id}
Authorization: Bearer <access_token>
```

---

## User Profile

### Get Profile
```http
GET /users/me
Authorization: Bearer <access_token>
```

### Update Profile
```http
PATCH /users/me
Authorization: Bearer <access_token>
```

Request:
```json
{
  "name": "Updated Name"
}
```

---

## Organizations

### Create Organization
```http
POST /organizations
Authorization: Bearer <access_token>
```

Request:
```json
{
  "name": "Acme Inc",
  "slug": "acme"
}
```

---

### Add Member
```http
POST /organizations/{organization_id}/members
Authorization: Bearer <access_token>
```

---

### List Members
```http
GET /organizations/{organization_id}/members
Authorization: Bearer <access_token>
```

---

### Remove Member
```http
DELETE /organizations/{organization_id}/members/{user_id}
Authorization: Bearer <access_token>
```

---

## Roles

### Create Role
```http
POST /organizations/{organization_id}/roles
Authorization: Bearer <access_token>
```

### Assign Permission to Role
```http
POST /organizations/{organization_id}/roles/{role_id}/permissions
Authorization: Bearer <access_token>
```

### Assign Role to Member
```http
POST /organizations/{organization_id}/members/{user_id}/roles
Authorization: Bearer <access_token>
```

---

## API Keys

### Create API Key
```http
POST /organizations/{organization_id}/api-keys
Authorization: Bearer <access_token>
```

### List API Keys
```http
GET /organizations/{organization_id}/api-keys
Authorization: Bearer <access_token>
```

### Revoke API Key
```http
DELETE /organizations/{organization_id}/api-keys/{key_id}
Authorization: Bearer <access_token>
```

---

## Error Responses

```http
400 Bad Request
401 Unauthorized
403 Forbidden
404 Not Found
409 Conflict
500 Internal Server Error
```
