-- todo clean up missing references (map -> run etc)

CREATE TABLE trees
(
    id   INTEGER PRIMARY KEY NOT NULL,
    sha1 BLOB UNIQUE         NOT NULL
-- todo what else does git store about a working tree?
-- todo could also keep commit info as optional fields
);

CREATE TABLE runs
(
    id           INTEGER PRIMARY KEY NOT NULL,
    tree_id      INTEGER,
    make_command TEXT,
    began        DATETIME            NOT NULL,
    ended        DATETIME            NOT NULL,
    outcome      INTEGER             NOT NULL,

    CONSTRAINT fk_runs_trees
        FOREIGN KEY (tree_id)
            REFERENCES trees (id)
            ON DELETE SET NULL
);

CREATE TABLE files
(
    id           INTEGER PRIMARY KEY NOT NULL,
    sha256       BLOB UNIQUE         NOT NULL,
    path         TEXT                NOT NULL COLLATE NOCASE,
    size         INTEGER             NOT NULL,
    created      DATETIME            NOT NULL,
    modified     DATETIME            NOT NULL,
    accessed     DATETIME            NOT NULL,
    is_generated BOOLEAN             NOT NULL
);

CREATE TABLE files_trees
(
    file_id INTEGER NOT NULL,
    tree_id INTEGER NOT NULL,
    PRIMARY KEY (file_id, tree_id),

    CONSTRAINT fk_files_trees_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_files_trees_trees
        FOREIGN KEY (tree_id)
            REFERENCES trees (id)
            ON DELETE CASCADE
);

CREATE TABLE makefiles
(
    id      INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER             NOT NULL,

    CONSTRAINT fk_makefiles_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE
);

CREATE TABLE dol_files
(
    id      INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER             NOT NULL,

    CONSTRAINT fk_dol_files
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
            REFERENCES dolphins (id)
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
    id   INTEGER PRIMARY KEY NOT NULL,
    name TEXT                NOT NULL COLLATE BINARY
    -- todo 
);

-- todo object files

CREATE TABLE map_symbols
(
    id              INTEGER PRIMARY KEY NOT NULL,
    symbol_id       INTEGER UNIQUE      NOT NULL,
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
    id          INTEGER PRIMARY KEY NOT NULL,
    symbol_id   INTEGER UNIQUE      NOT NULL,
    elf_file_id INTEGER             NOT NULL,

    CONSTRAINT fk_elf_symbols_symbols
        FOREIGN KEY (symbol_id)
            REFERENCES symbols (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_elf_symbols_elf_files
        FOREIGN KEY (elf_file_id)
            REFERENCES elf_files (id)
            ON DELETE CASCADE
);

CREATE TABLE object_files
(
    id          INTEGER PRIMARY KEY NOT NULL,
    file_id     INTEGER             NOT NULL,
    map_file_id INTEGER             NOT NULL,

    CONSTRAINT fk_objects_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_objects_map_files
        FOREIGN KEY (map_file_id)
            REFERENCES map_files (id)
            ON DELETE SET NULL
);

CREATE TABLE c_files
(
    id             INTEGER PRIMARY KEY NOT NULL,
    file_id        INTEGER             NOT NULL,
    object_file_id INTEGER,
    is_generated   BOOLEAN             NOT NULL,

    CONSTRAINT fk_c_files_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_c_files_object_files
        FOREIGN KEY (object_file_id)
            REFERENCES object_files (id)
            ON DELETE SET NULL
);

CREATE TABLE asm_files
(
    id      INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER             NOT NULL
);

CREATE TABLE html_files
(
    id           INTEGER PRIMARY KEY NOT NULL,
    file_id      INTEGER             NOT NULL,
    is_generated BOOLEAN             NOT NULL,

    CONSTRAINT fk_html_files_files
        FOREIGN KEY (file_id)
            REFERENCES files (id)
            ON DELETE CASCADE
);