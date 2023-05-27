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
   corresponding_question integer REFERENCES questions
);
```

Check tables creation:

```bash
\dt
```

To delete tables:

```bash
drop table answers, questions;
```

