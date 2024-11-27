CREATE TABLE IF NOT EXISTS records.bookings (
    user_id uuid REFERENCES auth.users(id) ON DELETE CASCADE,
    form_id uuid NOT NULL REFERENCES records.session_forms(id) ON DELETE CASCADE,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, form_id)
);
