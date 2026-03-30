-- Bootstrap admin account (password verified by app via password-auth / Argon2id).
-- Username: admin  Password: Never deploy with default passwords.
-- Replace credentials in production; see docs/quickstart.md and docs/deploy.md.

INSERT INTO users (email, display_name, username, passhash, admin_flag)
VALUES (
    'admin@localhost',
    'Admin',
    'admin',
    '$argon2id$v=19$m=19456,t=2,p=1$M9pSDD5zgrIofnt7imVWyA$wtPsdh4godDKhk1nnkLiGy7VhG2FUduTkxVW7WuJ/JM',
    true
)
ON CONFLICT (username) DO NOTHING;
