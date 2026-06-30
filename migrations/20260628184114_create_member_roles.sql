CREATE TABLE member_roles (
    membership_id UUID NOT NULL
        REFERENCES memberships(id)
        ON DELETE CASCADE,

    role_id UUID NOT NULL
        REFERENCES roles(id)
        ON DELETE CASCADE,

    PRIMARY KEY (
        membership_id,
        role_id
    )
);