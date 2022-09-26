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
        object_file_id -> Nullable<Integer>,
        is_generated -> Bool,
    }
}

diesel::table! {
    dol_files (id) {
        id -> Integer,
        file_id -> Integer,
    }
}

diesel::table! {
    elf_files (id) {
        id -> Integer,
        file_id -> Integer,
        dol_file_id -> Nullable<Integer>,
    }
}

diesel::table! {
    elf_symbols (id) {
        id -> Integer,
        symbol_id -> Integer,
        elf_file_id -> Integer,
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
    makefiles (id) {
        id -> Integer,
        file_id -> Integer,
    }
}

diesel::table! {
    map_files (id) {
        id -> Integer,
        file_id -> Integer,
        elf_file_id -> Nullable<Integer>,
    }
}

diesel::table! {
    map_symbols (id) {
        id -> Integer,
        symbol_id -> Integer,
        map_file_id -> Integer,
        parent_id -> Nullable<Integer>,
        virtual_address -> Integer,
        section_offset -> Integer,
        file_offset -> Integer,
        size -> Integer,
        depth -> Integer,
        #[sql_name = "type"]
        type_ -> Integer,
        scope -> Integer,
    }
}

diesel::table! {
    object_files (id) {
        id -> Integer,
        file_id -> Integer,
        map_file_id -> Integer,
    }
}

diesel::table! {
    runs (id) {
        id -> Integer,
        tree_id -> Nullable<Integer>,
        make_command -> Nullable<Text>,
        began -> Timestamp,
        ended -> Timestamp,
        outcome -> Integer,
    }
}

diesel::table! {
    symbols (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    trees (id) {
        id -> Integer,
        sha1 -> Binary,
    }
}

diesel::joinable!(c_files -> files (file_id));
diesel::joinable!(c_files -> object_files (object_file_id));
diesel::joinable!(dol_files -> files (file_id));
diesel::joinable!(elf_files -> files (file_id));
diesel::joinable!(elf_symbols -> elf_files (elf_file_id));
diesel::joinable!(elf_symbols -> symbols (symbol_id));
diesel::joinable!(files_trees -> files (file_id));
diesel::joinable!(files_trees -> trees (tree_id));
diesel::joinable!(html_files -> files (file_id));
diesel::joinable!(makefiles -> files (file_id));
diesel::joinable!(map_files -> elf_files (elf_file_id));
diesel::joinable!(map_files -> files (file_id));
diesel::joinable!(map_symbols -> map_files (map_file_id));
diesel::joinable!(map_symbols -> symbols (symbol_id));
diesel::joinable!(object_files -> files (file_id));
diesel::joinable!(object_files -> map_files (map_file_id));
diesel::joinable!(runs -> trees (tree_id));

diesel::allow_tables_to_appear_in_same_query!(
    asm_files,
    c_files,
    dol_files,
    elf_files,
    elf_symbols,
    files,
    files_trees,
    html_files,
    makefiles,
    map_files,
    map_symbols,
    object_files,
    runs,
    symbols,
    trees,
);
