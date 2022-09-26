// @generated automatically by Diesel CLI.

diesel::table! {
    asm_files (id) {
        id -> Integer,
        file_id -> Integer,
    }
}

diesel::table! {
    c_files (id) {
        id -> Integer,
        file_id -> Integer,
        is_generated -> Bool,
    }
}

diesel::table! {
    dolphins (id) {
        id -> Integer,
        file_id -> Nullable<Integer>,
    }
}

diesel::table! {
    elf_sections (id) {
        id -> Integer,
        elf_id -> Integer,
        name -> Text,
        file_offset -> Integer,
        virtual_address -> Integer,
        size -> Integer,
    }
}

diesel::table! {
    elf_symbols (id) {
        id -> Integer,
        symbol_id -> Integer,
    }
}

diesel::table! {
    elves (id) {
        id -> Integer,
        file_id -> Integer,
        dolphin_id -> Integer,
    }
}

diesel::table! {
    files (id) {
        id -> Integer,
        sha256 -> Binary,
        path -> Text,
        size -> Integer,
        created -> Timestamp,
        modified -> Timestamp,
        accessed -> Timestamp,
        is_generated -> Bool,
    }
}

diesel::table! {
    files_trees (file_id, tree_id) {
        file_id -> Integer,
        tree_id -> Integer,
    }
}

diesel::table! {
    html_files (id) {
        id -> Integer,
        file_id -> Integer,
        is_generated -> Bool,
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
    makefiles (id) {
        id -> Integer,
        file_id -> Integer,
    }
}

diesel::table! {
    maps (id) {
        id -> Integer,
        file_id -> Integer,
    }
}

diesel::table! {
    runs (id) {
        id -> Integer,
        tree_id -> Nullable<Integer>,
        began -> Timestamp,
        ended -> Timestamp,
        outcome -> Integer,
    }
}

diesel::table! {
    symbols (id) {
        id -> Integer,
        elf_section_id -> Integer,
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

diesel::joinable!(elf_symbols -> symbols (symbol_id));
diesel::joinable!(files_trees -> files (file_id));
diesel::joinable!(files_trees -> trees (tree_id));
diesel::joinable!(html_files -> files (file_id));
diesel::joinable!(makefiles -> files (file_id));
diesel::joinable!(runs -> trees (tree_id));

diesel::allow_tables_to_appear_in_same_query!(
    asm_files,
    c_files,
    dolphins,
    elf_sections,
    elf_symbols,
    elves,
    files,
    files_trees,
    html_files,
    linker_symbols,
    makefiles,
    maps,
    runs,
    symbols,
    trees,
);
