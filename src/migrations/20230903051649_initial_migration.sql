CREATE TABLE IF NOT EXISTS games (
    id TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL UNIQUE,
    platforms TEXT NOT NULL
);
INSERT INTO games
VALUES(
        '9814c939-071c-4841-aaf7-091a4efda4d0',
        'Ghostrunner',
        'pc,switch'
    ),
    (
        'caa05adb-aac5-4956-a52d-8ce63ae6acc5',
        'Metroid: Dread',
        "switch"
    );
