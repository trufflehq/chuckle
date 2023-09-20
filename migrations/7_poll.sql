create table if not exists poll (
	id uuid primary key not null default gen_random_uuid(),
	guild_id text not null,
	channel_id text not null,
	message_id text not null,
	creator_id text not null,
	votes_per_user integer not null,
	created_at timestamptz not null default now()
);

create table if not exists poll_option (
	id uuid primary key not null default gen_random_uuid(),
	poll_id uuid not null references poll(id) on delete cascade,
	option integer not null,
	created_at timestamptz not null default now()
);

create table if not exists poll_option_vote (
	id uuid primary key not null default gen_random_uuid(),
	poll_id uuid not null references poll(id) on delete cascade,
	poll_option_id uuid not null references poll_option(id) on delete cascade,
	user_id text not null,
	created_at timestamptz not null default now()
);
