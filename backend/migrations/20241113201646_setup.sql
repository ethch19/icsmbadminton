CREATE SCHEMA IF NOT EXISTS auth;
CREATE SCHEMA IF NOT EXISTS records;

CREATE TABLE IF NOT EXISTS auth.tiers (
    tier smallint PRIMARY KEY,
    name text UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS auth.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name varchar(30) NOT NULL,
    surname varchar(30) NOT NULL,
    shortcode varchar(20) UNIQUE NOT NULL,
    cid int UNIQUE NOT NULL,
    password text NOT NULL,
    admin bool NOT NULL,
    tier smallint NOT NULL REFERENCES auth.tiers(tier) ON DELETE RESTRICT,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS auth.pending_users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    verification_token UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    first_name varchar(30) NOT NULL,
    surname varchar(30) NOT NULL,
    shortcode varchar(20) UNIQUE NOT NULL,
    cid int UNIQUE NOT NULL,
    password text NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS records.session_forms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    title varchar(50) NOT NULL,
    description text NOT NULL,
    location text NOT NULL,
    tier smallint NOT NULL REFERENCES auth.tiers(tier) ON DELETE RESTRICT,
    start_time timestamp with time zone NOT NULL,
    end_time timestamp with time zone NOT NULL,
    recurrence interval,
    recurrence_end timestamp with time zone,
    user_limit smallint DEFAULT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_recurrence_filled CHECK (
        (recurrence IS NOT NULL AND recurrence_end IS NOT NULL) OR
            (recurrence IS NULL AND recurrence_end IS NULL)
        )
);

CREATE TABLE IF NOT EXISTS records.members (
    first_name text,
    surname text,
    cid text PRIMARY KEY,
    email text,
    login text,
    order_no int,
    member_type text
);

INSERT INTO auth.tiers VALUES
    (0, 'Non-Member'),
    (1, 'Member'),
    (2, 'Team Member');
