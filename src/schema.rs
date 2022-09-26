// @generated automatically by Diesel CLI.

diesel::table! {
    builds (id) {
        id -> Integer,
        git_tree_id -> Nullable<Integer>,
        path_root -> Nullable<Text>,
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
    dol_files (id) {
        id -> Integer,
        file_id -> Integer,
        sha1 -> Binary,
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
        symbol_name_id -> Integer,
        file_offset -> Integer,
        size -> Integer,
        hash -> Binary,
        fuzzy_hash -> Nullable<Binary>,
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
        compression_algorithm -> Integer,
        compression_level -> Integer,
        compressed_size -> Integer,
    }
}

diesel::table! {
    files_git_trees (file_id, git_tree_id) {
        file_id -> Integer,
        git_tree_id -> Integer,
    }
}

diesel::table! {
    git_trees (id) {
        id -> Integer,
        sha1 -> Binary,
    }
}

diesel::table! {
    h_files (id) {
        id -> Integer,
        file_id -> Integer,
    }
}

diesel::table! {
    h_files_c_files (h_file_id, c_file_id) {
        h_file_id -> Integer,
        c_file_id -> Integer,
    }
}

diesel::table! {
    html_files (id) {
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
    mk_files (id) {
        id -> Integer,
        file_id -> Integer,
    }
}

diesel::table! {
    o_files (id) {
        id -> Integer,
        file_id -> Integer,
        map_file_id -> Integer,
    }
}

diesel::table! {
    runs (id) {
        id -> Integer,
        git_tree_id -> Nullable<Integer>,
        command -> Text,
        began -> Timestamp,
        ended -> Timestamp,
        error_code -> Integer,
    }
}

diesel::table! {
    s_files (id) {
        id -> Integer,
        file_id -> Integer,
    }
}

diesel::table! {
    symbol_names (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    symbols (id) {
        id -> Integer,
    }
}

diesel::joinable!(builds -> git_trees (git_tree_id));
diesel::joinable!(c_files -> files (file_id));
diesel::joinable!(dol_files -> files (file_id));
diesel::joinable!(elf_files -> dol_files (dol_file_id));
diesel::joinable!(elf_files -> files (file_id));
diesel::joinable!(elf_symbols -> elf_files (elf_file_id));
diesel::joinable!(elf_symbols -> symbol_names (symbol_name_id));
diesel::joinable!(elf_symbols -> symbols (symbol_id));
diesel::joinable!(files_git_trees -> files (file_id));
diesel::joinable!(files_git_trees -> git_trees (git_tree_id));
diesel::joinable!(h_files -> files (file_id));
diesel::joinable!(h_files_c_files -> c_files (c_file_id));
diesel::joinable!(h_files_c_files -> h_files (h_file_id));
diesel::joinable!(html_files -> files (file_id));
diesel::joinable!(map_files -> elf_files (elf_file_id));
diesel::joinable!(map_files -> files (file_id));
diesel::joinable!(map_symbols -> map_files (map_file_id));
diesel::joinable!(map_symbols -> symbols (symbol_id));
diesel::joinable!(mk_files -> files (file_id));
diesel::joinable!(o_files -> files (file_id));
diesel::joinable!(o_files -> map_files (map_file_id));
diesel::joinable!(runs -> git_trees (git_tree_id));
diesel::joinable!(s_files -> files (file_id));

diesel::allow_tables_to_appear_in_same_query!(
    builds,
    c_files,
    dol_files,
    elf_files,
    elf_symbols,
    files,
    files_git_trees,
    git_trees,
    h_files,
    h_files_c_files,
    html_files,
    map_files,
    map_symbols,
    mk_files,
    o_files,
    runs,
    s_files,
    symbol_names,
    symbols,
);
