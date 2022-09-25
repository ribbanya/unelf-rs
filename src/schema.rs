// @generated automatically by Diesel CLI.

diesel::table! {
    elves (id) {
        id -> Integer,
        file_id -> Integer,
    }
}

diesel::table! {
    files (id) {
        id -> Integer,
        sha256 -> Binary,
        path -> Text,
    }
}

diesel::table! {
    linker_symbols (id) {
        id -> Integer,
        elf_id -> Integer,
        name -> Text,
        virtual_address -> Integer,
    }
}

diesel::table! {
    runs (id) {
        id -> Integer,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    sections (id) {
        id -> Integer,
        elf_id -> Integer,
        name -> Text,
        file_offset -> Integer,
        virtual_address -> Integer,
        size -> Integer,
    }
}

diesel::table! {
    symbols (id) {
        id -> Integer,
        section_id -> Integer,
        name -> Text,
        virtual_address -> Integer,
        section_offset -> Integer,
        file_offset -> Integer,
        size -> Integer,
        parent_id -> Nullable<Integer>,
        depth -> Integer,
        #[sql_name = "type"]
        type_ -> Integer,
        scope -> Integer,
    }
}

diesel::table! {
    trees (id) {
        id -> Integer,
        sha1 -> Binary,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    elves,
    files,
    linker_symbols,
    runs,
    sections,
    symbols,
    trees,
);
