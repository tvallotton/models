# Models CLI

## Installation
To install the CLI use the following command: 
```
$ cargo install models-cli
```

## Usage
There are three main commands: `database`, `generate` and `migrate`. 

### database
it can be abbreviated as `db`. It includes the subcomands:
* `create`: Creates the database specified in your DATABASE_URL.
* `drop`: Drops the database specified in your DATABASE_URL.
* `reset`: Drops the database specified in your DATABASE_URL, re-creates it, and runs any pending migrations.
* `setup`: Creates the database specified in your DATABASE_URL and runs any pending migrations.

### generate
It is used to generate migrations. It can be used to generate down migrations as well if the `-r` flag is enabled. 
The `--source` variable can be used to specify the migrations directory. 
The `--table` variable can be used to filter the names of the tables to target in the generation. 

### migrate
* `add`: Create a new migration with the given description, and the current time as the version.
* `info`: List all available migrations and their status.
* `revert`: Revert the latest migration with a down file.
* `run`: Run all pending migrations.