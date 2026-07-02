# DATABASE_SCHEMA

## Database Tables

### users
| Column | Type |
|--------|------|
| id | UUID |
| email | TEXT UNIQUE |
| password_hash | TEXT |
| name | TEXT |
| created_at | TIMESTAMPTZ |

---

### organizations
| Column | Type |
|--------|------|
| id | UUID |
| name | TEXT |
| slug | TEXT UNIQUE |
| created_at | TIMESTAMPTZ |

---

### memberships
| Column | Type |
|--------|------|
| user_id | UUID FK(users.id) |
| organization_id | UUID FK(organizations.id) |
| created_at | TIMESTAMPTZ |

Constraint:
```text
UNIQUE(user_id, organization_id)
```

---

### roles
| Column | Type |
|--------|------|
| id | UUID |
| organization_id | UUID FK(organizations.id) |
| name | TEXT |
| description | TEXT |
| created_at | TIMESTAMPTZ |

---

### permissions
| Column | Type |
|--------|------|
| id | UUID |
| name | TEXT UNIQUE |
| description | TEXT |

---

### role_permissions
| Column | Type |
|--------|------|
| role_id | UUID FK(roles.id) |
| permission_id | UUID FK(permissions.id) |

Constraint:
```text
UNIQUE(role_id, permission_id)
```

---

### member_roles
| Column | Type |
|--------|------|
| membership_id | UUID FK(memberships.id) |
| role_id | UUID FK(roles.id) |

Constraint:
```text
UNIQUE(membership_id, role_id)
```

---

### sessions
| Column | Type |
|--------|------|
| id | UUID |
| user_id | UUID FK(users.id) |
| refresh_token_hash | TEXT |
| user_agent | TEXT NULL |
| ip_address | TEXT NULL |
| expires_at | TIMESTAMPTZ |
| created_at | TIMESTAMPTZ |

---

### api_keys
| Column | Type |
|--------|------|
| id | UUID |
| organization_id | UUID FK(organizations.id) |
| name | TEXT |
| key_hash | TEXT |
| created_by | UUID FK(users.id) |
| expires_at | TIMESTAMPTZ NULL |
| last_used_at | TIMESTAMPTZ NULL |
| created_at | TIMESTAMPTZ |

API keys are stored only as hashes.

---

### audit_logs
| Column | Type |
|--------|------|
| id | UUID |
| organization_id | UUID NULL |
| actor_id | UUID NULL |
| action | TEXT |
| resource_type | TEXT |
| resource_id | UUID NULL |
| created_at | TIMESTAMPTZ |

---

## Entity Relationship Diagram

```text
Users
  │
  └── Memberships ─── Organizations
                         │
                         ├── Roles ─── RolePermissions ─── Permissions
                         │
                         ├── API Keys
                         │
                         └── Audit Logs

Users
  ├── Sessions
  └── Memberships ─── MemberRoles ─── Roles
```

## Design Decisions

- UUIDs used as primary keys.
- Many-to-many relationships modeled using join tables.
- Unique constraints prevent duplicate memberships and role assignments.
- Refresh tokens and API keys are stored only as hashes.
- Audit logs provide traceability of critical actions.
