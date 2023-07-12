create table if not exists "user" (
	id uuid primary key not null default gen_random_uuid(),
	discord_id text,
	-- such as 45381083
	github_id int,
	created_at timestamptz not null default now()
);

create table if not exists pr_review_output (
	id uuid primary key not null default gen_random_uuid(),
	pr_number int not null,
	repo_owner text not null,
	repo text not null,
	thread_id text not null,
	created_at timestamptz not null default now()
);
