CREATE TABLE trees
(
    id   INTEGER PRIMARY KEY NOT NULL,
    sha1 BLOB UNIQUE         NOT NULL
-- todo what else does git store about a working tree?
-- todo could also keep commit info as optional fields
);

CREATE TABLE runs
(
    id        INTEGER PRIMARY KEY NOT NULL,
    timestamp DATETIME            NOT NULL
-- todo elapsed?
-- todo run <-> tree join table
-- todo run <-> file join table
);

CREATE TABLE files
(
    id     INTEGER PRIMARY KEY NOT NULL,
    sha256 BLOB UNIQUE         NOT NULL,
    path   TEXT                NOT NULL COLLATE NOCASE
-- todo tree <-> file join table
-- todo file creation date
-- todo file modified date
-- todo last accessed for pruning?
);

CREATE TABLE elves
(
    id      INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER             NOT NULL
);

CREATE TABLE sections
(
    id              INTEGER PRIMARY KEY NOT NULL,
    elf_id          INTEGER             NOT NULL,
    name            TEXT                NOT NULL COLLATE BINARY,
    file_offset     INTEGER             NOT NULL,
    virtual_address INTEGER             NOT NULL,
    size            INTEGER             NOT NULL
);

CREATE TABLE symbols
(
    id              INTEGER PRIMARY KEY NOT NULL,
    section_id      INTEGER             NOT NULL,
    name            TEXT                NOT NULL COLLATE BINARY,
    virtual_address INTEGER             NOT NULL,
    section_offset  INTEGER             NOT NULL,
    file_offset     INTEGER             NOT NULL,
    size            INTEGER             NOT NULL,
    parent_id       INTEGER,
    depth           INTEGER             NOT NULL,
    type            INTEGER             NOT NULL,
    scope           INTEGER             NOT NULL
);

CREATE TABLE linker_symbols
(
    id              INTEGER PRIMARY KEY NOT NULL,
    elf_id          INTEGER             NOT NULL,
    name            TEXT                NOT NULL COLLATE BINARY,
    virtual_address INTEGER             NOT NULL
);

-- todo source files
-- todo makefiles?
-- todo dols!
-- todo generated html files