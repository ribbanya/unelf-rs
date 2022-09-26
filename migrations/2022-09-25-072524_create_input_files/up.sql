CREATE TABLE git_trees
(
    id   INTEGER PRIMARY KEY NOT NULL,
    sha1 BLOB                NOT NULL
-- todo what else does git store about a working tree?
-- todo could also keep commit info as optional fields
);

CREATE TABLE runs
(
    id          INTEGER PRIMARY KEY NOT NULL,
    git_tree_id INTEGER,
    command     TEXT                NOT NULL,
    began       DATETIME            NOT NULL,
    ended       DATETIME            NOT NULL,
    error_code  INTEGER             NOT NULL,

    CONSTRAINT fk_runs_git_trees
        FOREIGN KEY (git_tree_id)
            REFERENCES git_trees (id)
            ON DELETE SET NULL
);

CREATE TABLE files
(
    id                    INTEGER PRIMARY KEY NOT NULL,
    sha256                BLOB                NOT NULL,

    -- todo this is not actually attached to the cached file at all
    path                  TEXT                NOT NULL COLLATE NOCASE,

    size                  INTEGER             NOT NULL,
    created               DATETIME            NOT NULL,
    modified              DATETIME            NOT NULL,
    accessed              DATETIME            NOT NULL,
    is_generated          BOOLEAN             NOT NULL,
    compression_algorithm INTEGER             NOT NULL,
    compression_level     INTEGER             NOT NULL,
    compressed_size       INTEGER             NOT NULL
);

CREATE TABLE files_git_trees
(
    file_id     INTEGER NOT NULL,
    git_tree_id INTEGER NOT NULL,
    PRIMARY KEY (file_id, git_tree_id),

    CONSTRAINT fk_files_git_trees_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_files_git_trees_git_trees
        FOREIGN KEY (git_tree_id)
            REFERENCES git_trees (id)
            ON DELETE CASCADE
);

CREATE TABLE mk_files
(
    id      INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER             NOT NULL,

    CONSTRAINT fk_mk_files_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE
);

CREATE TABLE dol_files
(
    id      INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER             NOT NULL,
    sha1    BLOB                NOT NULL,

    CONSTRAINT fk_dol_files_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE
);

CREATE TABLE elf_files
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER             NOT NULL,
    dol_file_id INTEGER,

    CONSTRAINT fk_elf_files_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_elf_files_dol_files
        FOREIGN KEY (dol_file_id)
            REFERENCES dol_files (id)
            ON DELETE SET NULL
);

CREATE TABLE map_files
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER             NOT NULL,
    elf_file_id INTEGER,

    CONSTRAINT fk_map_files_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_map_files_elf_files
        FOREIGN KEY (elf_file_id)
            REFERENCES elf_files (id)
            ON DELETE SET NULL
);

CREATE TABLE symbols
(
    -- symbols have nothing in common with each other
    -- except their semantic identity
    -- dol symbols have no name, c symbols have no bytecode size, etc
    id INTEGER PRIMARY KEY NOT NULL
);

CREATE TABLE symbol_names
(
    id   INTEGER PRIMARY KEY NOT NULL,
    name TEXT UNIQUE         NOT NULL COLLATE BINARY
);

CREATE TABLE map_symbols
(
    id              INTEGER PRIMARY KEY NOT NULL,
    symbol_id       INTEGER             NOT NULL,
    map_file_id     INTEGER             NOT NULL,
    parent_id       INTEGER,
    virtual_address INTEGER             NOT NULL,
    section_offset  INTEGER             NOT NULL,
    file_offset     INTEGER             NOT NULL,
    size            INTEGER             NOT NULL,
    depth           INTEGER             NOT NULL,
    type            INTEGER             NOT NULL,
    scope           INTEGER             NOT NULL,

    CONSTRAINT fk_map_symbols_symbols
        FOREIGN KEY (symbol_id)
            REFERENCES symbols (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_map_symbols_map_files
        FOREIGN KEY (map_file_id)
            REFERENCES map_files (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_map_symbols_parent
        FOREIGN KEY (parent_id)
            REFERENCES map_symbols (id)
            ON DELETE SET NULL
);

CREATE TABLE elf_symbols
(
    id             INTEGER PRIMARY KEY NOT NULL,
    symbol_id      INTEGER             NOT NULL,
    elf_file_id    INTEGER             NOT NULL,
    symbol_name_id INTEGER             NOT NULL,
    file_offset    INTEGER             NOT NULL,
    size           INTEGER             NOT NULL,

    -- hashes are the symbol content if size < 256, otherwise sha256
    hash           BLOB                NOT NULL,

    -- fuzzy_hash is null if equal to hash
    fuzzy_hash     BLOB,

    CONSTRAINT fk_elf_symbols_symbols
        FOREIGN KEY (symbol_id)
            REFERENCES symbols (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_elf_symbols_symbol_names
        FOREIGN KEY (symbol_name_id)
            REFERENCES symbol_names (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_elf_symbols_elf_files
        FOREIGN KEY (elf_file_id)
            REFERENCES elf_files (id)
            ON DELETE CASCADE
);

CREATE TABLE builds
(
    id          INTEGER PRIMARY KEY NOT NULL,
    git_tree_id INTEGER,
    path_root   TEXT,
    -- todo timestamp, mk_file, mk_command, etc

    CONSTRAINT fk_builds_git_trees
        FOREIGN KEY (git_tree_id)
            REFERENCES git_trees (id)
            ON DELETE SET NULL
);

-- todo build<->o_file
-- todo run<->build<->source<->o

CREATE TABLE o_files
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER             NOT NULL,
    map_file_id INTEGER             NOT NULL,

    CONSTRAINT fk_o_files_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_o_files_map_files
        FOREIGN KEY (map_file_id)
            REFERENCES map_files (id)
            ON DELETE SET NULL
);

CREATE TABLE c_files
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER             NOT NULL,
    is_generated BOOLEAN             NOT NULL,

    CONSTRAINT fk_c_files_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE
);

-- todo dep_files

CREATE TABLE h_files_c_files
(
    h_file_id INTEGER NOT NULL,
    c_file_id INTEGER NOT NULL,
    PRIMARY KEY (h_file_id, c_file_id),

    CONSTRAINT fk_h_files_c_files_h_files
        FOREIGN KEY (h_file_id)
            REFERENCES h_files (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_h_files_c_files_c_files
        FOREIGN KEY (c_file_id)
            REFERENCES c_files (id)
            ON DELETE CASCADE
);

CREATE TABLE h_files
(
    id      INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER             NOT NULL,

    CONSTRAINT fk_h_files_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE
);

CREATE TABLE s_files
(
    id      INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER             NOT NULL,

    CONSTRAINT fk_s_files_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE
);

CREATE TABLE html_files
(
    id      INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER             NOT NULL,

    CONSTRAINT fk_html_files_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE
);