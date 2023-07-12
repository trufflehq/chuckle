create table if not exists settings (
	id uuid primary key not null default gen_random_uuid(),
	guild_id bigint not null,
	forum_log_channel_id bigint not null
);

create table if not exists notifications (
	id uuid primary key not null default gen_random_uuid(),
	user_id bigint not null,
	guild_id bigint not null,
	author_id bigint not null,
	channel_id bigint not null,
	message_id bigint not null,
	completed boolean not null default false,
	notify_at timestamptz not null,
	created_at timestamptz not null default now()
);
