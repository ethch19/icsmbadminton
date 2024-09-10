CREATE SCHEMA IF NOT EXISTS auth;
CREATE SCHEMA IF NOT EXISTS records;

CREATE TABLE IF NOT EXISTS auth.admins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username varchar(20) UNIQUE NOT NULL,
    password VARCHAR(100) NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    last_login timestamp with time zone NOT NULL
);

CREATE TABLE IF NOT EXISTS records.forms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES auth.admins(id) ON DELETE RESTRICT,
    title varchar(50) NOT NULL,
    subtitle varchar(50) NOT NULL,
    description text NOT NULL,
    sub_limit smallint DEFAULT NULL,
    immutable bool NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS records.fields (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    form_id uuid NOT NULL REFERENCES records.forms(id) ON DELETE CASCADE,
    title text NOT NULL,
    index int NOT NULL,
    mandatory bool NOT NULL
);

CREATE TABLE IF NOT EXISTS records.user_subs (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  form_id uuid NOT NULL REFERENCES records.forms(id) ON DELETE CASCADE,
  shortcode varchar(15) NOT NULL,
  name varchar(30) NOT NULL,
  created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS records.field_data (
  sub_id uuid REFERENCES records.user_subs(id) ON DELETE CASCADE,
  field_id uuid REFERENCES records.fields(id) ON DELETE CASCADE,
  data int NOT NULL,
  PRIMARY KEY (sub_id, field_id)
);
