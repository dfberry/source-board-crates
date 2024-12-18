diesel::table! {
    osb_user_custom_config (id) {
        id -> Text,
        user_id -> Text,
        repo_name -> Text,
        created_at -> Timestamptz
    }
}
diesel::table! {
    osb_github_logfiles (id) {
        id -> Uuid,
        logfile -> Jsonb,
        created_at -> Timestamptz,
        org_repo -> Text,
    }
}