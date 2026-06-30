CREATE INDEX idx_users_email
ON users(email);

CREATE INDEX idx_memberships_user_id
ON memberships(user_id);

CREATE INDEX idx_memberships_org_id
ON memberships(organization_id);

CREATE INDEX idx_roles_org_id
ON roles(organization_id);

CREATE INDEX idx_sessions_user_id
ON sessions(user_id);

CREATE INDEX idx_api_keys_org_id
ON api_keys(organization_id);

CREATE INDEX idx_audit_actor_id
ON audit_logs(actor_id);

CREATE INDEX idx_audit_org_id
ON audit_logs(organization_id);

CREATE INDEX idx_permissions_name
ON permissions(name);