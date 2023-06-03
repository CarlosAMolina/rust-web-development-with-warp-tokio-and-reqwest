## Steps

### Initial steps
First run the makefile commands to work with PostgreSQL.

### Database creation

After that, create the database:

```bash
make connect-psql
create database rustwebdev;
# List databases.
\l
# Connect to de database.
\c rustwebdev;
```

### Table creation

```sql
CREATE TABLE IF NOT EXISTS questions (
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
```

```sql
CREATE TABLE IF NOT EXISTS answers (
   id serial PRIMARY KEY,
   content TEXT NOT NULL,
   created_on TIMESTAMP NOT NULL DEFAULT NOW(),
   question_id integer REFERENCES questions
);
```

Check tables creation:

```bash
\dt
```

### Delete tables

```bash
drop table answers, questions;
```

## Migrations

### Configuration

#### sqlx-cli

To install:

```bash
cargo install sqlx-cli
```

#### Migrations creation

The database `rustwebdev` must be created previously.

Create migration

```bash
sqlx migrate add -r questions_table
```

Add in the *_questions_table.up.sql file:

```bash
CREATE TABLE IF NOT EXISTS questions (
   id serial PRIMARY KEY,
   title VARCHAR (255) NOT NULL,
   content TEXT NOT NULL,
   tags TEXT [],
   created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
```

Add in the *_questions_table.down.sql file:

```bash
DROP TABLE IF EXISTS questions;
```

Repeat the previous steps for the `answers` tables, you can see the results in the `migrations` folder.

### Run migrations

Connect to the database and run migration:

```bash
make connect-psql-db
# First, delete tables created previously.
drop table answers, questions;
# Exit the psql command.
```

To run the `*.up.sql` files in the `migrations` folder, we modified the `main.rs` file in the main server program to do it automatically but, to do it manually, run:

```bash
make run-migrations
```

The table `_sqlx_migrations` will be created to keep track of the already run migrations. This table must be deleted in some cases to avoid errors when recreating the other tables.

Run the `*.down.sql` files:

```bash
make revert-migrations
```

