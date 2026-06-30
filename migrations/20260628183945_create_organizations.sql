CREATE TABLE organizations (
    id UUID PRIMARY KEY
        DEFAULT gen_random_uuid(),

    name TEXT NOT NULL,

    slug TEXT NOT NULL UNIQUE,

    created_by UUID NOT NULL
        REFERENCES users(id)
        ON DELETE RESTRICT,

    created_at TIMESTAMPTZ
        NOT NULL
        DEFAULT NOW(),

    updated_at TIMESTAMPTZ
        NOT NULL
        DEFAULT NOW()
);