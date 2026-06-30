CREATE TABLE memberships (
    id UUID PRIMARY KEY
        DEFAULT gen_random_uuid(),

    user_id UUID NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE,

    organization_id UUID NOT NULL
        REFERENCES organizations(id)
        ON DELETE CASCADE,

    created_at TIMESTAMPTZ
        NOT NULL
        DEFAULT NOW(),

    UNIQUE (
        user_id,
        organization_id
    )
);