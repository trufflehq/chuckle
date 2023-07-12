create table if not exists modal (
	id uuid primary key not null default gen_random_uuid(),
	command text not null,
	meta jsonb,
	created_at timestamptz not null default now()
);
