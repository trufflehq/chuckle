create table if not exists custom_id (
	id uuid primary key not null,
	kind text not null,
	data jsonb not null,
	created_at timestamptz not null default now()
);
