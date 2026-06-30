CREATE TABLE api_keys (
    id UUID PRIMARY KEY
        DEFAULT gen_random_uuid(),

    organization_id UUID NOT NULL
        REFERENCES organizations(id)
        ON DELETE CASCADE,

    name TEXT NOT NULL,

    description TEXT,

    key_hash TEXT NOT NULL,

    expires_at TIMESTAMPTZ,

    last_used_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ
        NOT NULL
        DEFAULT NOW()
);