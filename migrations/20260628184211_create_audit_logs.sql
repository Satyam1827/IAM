CREATE TABLE audit_logs (
    id UUID PRIMARY KEY
        DEFAULT gen_random_uuid(),

    actor_id UUID
        REFERENCES users(id)
        ON DELETE SET NULL,

    organization_id UUID
        REFERENCES organizations(id)
        ON DELETE SET NULL,

    action TEXT NOT NULL,

    resource_type TEXT NOT NULL,

    resource_id UUID,

    metadata JSONB,

    created_at TIMESTAMPTZ
        NOT NULL
        DEFAULT NOW()
);