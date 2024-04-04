ALTER TABLE users ADD CHECK (password_hash <> '');
ALTER TABLE users ADD CHECK (username <> '');
ALTER TABLE users ADD CHECK (email <> '');
