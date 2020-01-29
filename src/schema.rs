table! {
    background_jobs (id) {
        id -> Int8,
        job_type -> Text,
        data -> Jsonb,
        retries -> Int4,
        last_retry -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    blogs (id) {
        id -> Int4,
        aname -> Varchar,
        avatar -> Varchar,
        intro -> Varchar,
        topic -> Varchar,
        blog_link -> Varchar,
        blog_host -> Varchar,
        tw_link -> Varchar,
        gh_link -> Varchar,
        other_link -> Varchar,
        is_top -> Bool,
        karma -> Int4,
    }
}

table! {
    items (id) {
        id -> Int4,
        title -> Varchar,
        slug -> Varchar,
        content -> Text,
        logo -> Varchar,
        author -> Varchar,
        ty -> Varchar,
        lang -> Varchar,
        topic -> Varchar,
        link -> Varchar,
        link_host -> Varchar,
        origin_link -> Varchar,
        post_by -> Varchar,
        post_at -> Timestamp,
        pub_at -> Date,
        is_top -> Bool,
        vote -> Int4,
    }
}

table! {
    itemtrans (origin_slug, trans_slug) {
        origin_slug -> Varchar,
        trans_slug -> Varchar,
        trans_lang -> Varchar,
        trans_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        uname -> Varchar,
        psw_hash -> Varchar,
        join_at -> Timestamp,
        last_seen -> Timestamp,
        avatar -> Varchar,
        email -> Varchar,
        link -> Varchar,
        intro -> Text,
        location -> Varchar,
        nickname -> Varchar,
        permission -> Int2,
        auth_from -> Varchar,
        email_confirmed -> Bool,
        karma -> Int4,
        is_pro -> Bool,
        can_push -> Bool,
        push_email -> Varchar,
    }
}

table! {
    voteitems (uname, item_id) {
        uname -> Varchar,
        item_id -> Int4,
        vote_at -> Timestamp,
        vote_as -> Int2,
    }
}

joinable!(voteitems -> items (item_id));

allow_tables_to_appear_in_same_query!(
    background_jobs,
    blogs,
    items,
    itemtrans,
    users,
    voteitems,
);
