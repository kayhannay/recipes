# Recipes

This is a very simple recipe database. It is not meant to be a full featured recipe database, though I use it at home. 
The main goal is to learn Rust and get some experience in the Rust eco-system. Don't expect perfect Rust code and best
practices here. I'm learning Rust and try things out. This is work in progress.

## Current stack

* Rust
* Rocket
* Handlebars
* Diesel
* MySQL
* Testcontainers

## How to run

Create a MySQL database and provide the connection string in an .env file in the project root:

```
DATABASE_URL=mysql://myuser:mysecret@127.0.0.1:3306/recipes
ROCKET_DATABASES={recipe_db={url="${DATABASE_URL}"}}
```
This is an example only, of course.

Now you can run the application with the following command:
```
cargo run
```

You can access the recipe database via http://localhost:8000 now.

There are no data in the database yet. You should create a user manually. After that you can log in and create 
recipes, categories, etc. The user password is stored as a SHA512 hash.

## Tests

There are some integration tests, which you can run by
```
cargo test
```
The integration tests use testcontainers for tests. So you need to have Docker installed.