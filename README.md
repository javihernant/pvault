# TODO
- Add support for new columns
    - write_update() OK
    - fetch() account OK
- Register
    - populate the account struct
    - hash password
- Stats command
    - add new data OK
    - give extra data to admins OK

PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "users" (
id INTEGER PRIMARY KEY AUTOINCREMENT,
username TEXT NOT NULL,
password TEXT NOT NULL,
motto TEXT DEFAULT '23-yo designer from San Francisco',
banned INTEGER NOT NULL DEFAULT 0,
retries_left INTEGER NOT NULL DEFAULT 3,
rank INTEGER NOT NULL DEFAULT 0,
email TEXT NOT NULL DEFAULT 'email@example.com',
last_on TEXT NOT NULL DEFAULT '1999-01-01 00:00:00',
status INTEGER NOT NULL DEFAULT 0,
signup_date TEXT NOT NULL DEFAULT '1999-01-01 00:00:00'
);
INSERT INTO users(id, username, password, motto, banned, retries_left, rank) VALUES(1,'javi','pass','im the staff',0,3,0);
INSERT INTO users(id, username, password, motto, banned, retries_left, rank) VALUES(2,'admin','pass','i am god',0,3,1);
COMMIT;
