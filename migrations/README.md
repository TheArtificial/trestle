### Migrations

Up migrations (apply in order):

1. [`20250309000000_trestle.up.sql`](20250309000000_trestle.up.sql) — users, invites, email, OAuth tables.
2. [`20250325000000_seed_admin_user.up.sql`](20250325000000_seed_admin_user.up.sql) — initial **admin** user (`admin` / documented password); safe to skip insert if `username` already exists (`ON CONFLICT DO NOTHING`).
