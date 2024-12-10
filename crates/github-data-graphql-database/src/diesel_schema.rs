diesel::table! {
    osb_user_custom_config (id) {
        id -> Text,
        user_id -> Text,
        repo_name -> Text,
        created_at -> Timestamptz
    }
}