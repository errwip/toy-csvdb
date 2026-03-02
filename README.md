# toy-db
This repo is a little toy **DataBase** I made to learn file editing in **Rust**.
It's basically a CSV file being read and written to by CLI commands.
For parsing the commands I use [clap-rs](https://github.com/clap-rs/clap) as dependancy.

For now the **db** has fixed table:
> Id, Activity, Date, Comment

Commas in comments are not properly parsed.

Currently the db supports commands / features:
---
- Read: `'read all'` or `'read <activity>'`
- Add: `'add <activity> <date> <comment>'`
- Remove: `'remove <id>'`
- Reindex: `'reindex'`
	> Reindexing takes the entire 'tabele' and sets the indexes from 1 to end of table.
	> Requiers confirmation as normaly it's bad practice to reindex a DataBase like this.
