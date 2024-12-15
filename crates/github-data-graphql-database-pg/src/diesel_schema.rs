// @generated automatically by Diesel CLI.

diesel::table! {
    logfiles (id) {
        id -> Uuid,
        logfile -> Jsonb,
        created_at -> Timestamptz,
        org_repo -> Text,
    }
}

diesel::table! {
    osb_user_custom_config (id) {
        id -> Text,
        user_id -> Text,
        repo_name -> Text,
        created_at -> Timestamptz
    }
}