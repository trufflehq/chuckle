create table if not exists hexil (
	id uuid primary key not null default gen_random_uuid(),
	guild_id text not null,
	user_id text not null,
	role_id text not null,
	created_at timestamptz not null default now()
);
