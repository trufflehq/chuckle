create table if not exists guild_settings (
	id uuid primary key not null default gen_random_uuid(),
	guild_id text not null,
	forum_log_channel_id text,
	default_repository text,
	default_repository_owner text,
	created_at timestamptz not null default now()
);
