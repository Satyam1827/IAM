-- Add migration script here
ALTER TABLE api_keys
ADD COLUMN created_by UUID
REFERENCES users(id)
ON DELETE SET NULL;