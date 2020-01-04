# Rust + GraphQL + Juniper + Diesel + Postgres + Actix

Yes, I know that this is a borderline absurd web stack for the ubiquitous TODO application but I had a *lot* of trouble getting this all to work. I started using these things for a more ambitious project and I'd love to spare you the trouble. So here's some basic boilerplate to get you up and running.

## Components

Here's what does what:

Component | Tool/lib
:---------|:--------
Web server | [actix-web](https://github.com/actix/actix-web)
Database | [PostgreSQL](https://postgresql.org)
SQL engine | [Diesel](https://diesel.rs)
GraphQL library | [Juniper](https://github.com/graphql-rust/juniper)
GraphQL UI | [GraphQL Playground](https://github.com/prisma-labs/graphql-playground)

## Run locally

> Before you get started, make sure that you have [PostgreSQL](https://postgresql.org), [Rust](https://rust-lang.org), [Cargo](https://doc.rust-lang.org/cargo/), and the [Diesel](https://diesel.rs) CLI installed and that you have Postgres running somewhere.

```bash
# Fetch the repo
git clone https://github.com/lucperkins/rust-actix-diesel-postgres-juniper
cd rust-actix-diesel-postgres-juniper

# Set up the database
cp .env.example .env # Modify this file to match your Postgres installation

diesel setup
diesel migration run

cargo run # could take a while!
```

> The `DATABASE_URL` can be any Postgres installation. For my purposes, I have it set to `postgres://localhost:5432/todos`.

Once the server is running, you can access the GraphQL Playground UI at http://localhost:4000/graphql.

## Schema

The server implements the following GraphQL schema:

```graphql
type Todo {
  id: ID!
  task: String!
  done: Boolean!
}

input CreateTodoInput {
  task: String!
  done: Boolean
}

type Query {
  allTodos: [Todo!]!
  getTodoById(id: Int): Todo
}

type Mutation {
  createTodo(input: CreateTodoInput): Todo
  markTodoAsDone(id: Int): Todo
  markTodoAsNotDone(id: Int): Todo
}

schema {
  Query
  Mutation
}
```

## Tour of the codebase

File | What it provides
:----|:----------------
[`context.rs`](./src/context.rs) | The GraphQL [context](https://graphql.org/learn/execution) that handles query execution
[`data.rs`](./src/data.rs) | A `Todos` struct and some helper functions encapsulate the [Diesel](https://diesel.rs)-powered Postgres querying logic
[`db.rs`](./src/db.rs) | The connection pool that handles the Postgres connection
[`endpoints.rs`](./src/endpoints.rs) | The `/graphql` HTTP endpoint that makes GraphQL and the GraphQL Playground work
[`graphql.rs`](./src/graphql.rs) | The `Query`, `Mutation`, and `Schema` objects that undergird the GraphQL interface
[`lib.rs`](./src/lib.rs) | Just the standard `lib.rs`
[`main.rs`](./src/main.rs) | [Actix](https://actix.rs) HTTP server setup
[`models.rs`](./src/models.rs) | All of the data types used for querying Postgres and providing GraphQL results
[`schema.rs`](./src/schema.rs) | The Diesel-generated table schema

## Future TODOs

Get it? Anyway, here's some areas for improvement (pull requests very much welcome):

* **Error handling** — Right now errors basically propagate directly from Diesel/Postgres into the GraphQL JSON output, which is subpar. If any of you can point me to good educational resources on this, please file an issue!
* **Better execution engine** — The server uses the extremely powerful [actix-web](https://github.com/actix/actix-web) but the actual DB interactions don't use Actix actors and it'd take this setup to the next level if they did.
* **Use macros for schema generation** — The powerful [`juniper_from_schema`](https://docs.rs/juniper-from-schema/0.5.1/juniper_from_schema/) macro could help reduce boilerplate and improve development velocity.

## Acknowledgments

I'm basically a beginner with Rust and would not have been able to put this together without peeking long and hard at the example projects and blog posts listed below. The lower-level bits you see here are basically stolen from [BrendanBall](https://github.com/BrendanBall). All that I've added is the [`Todos`](./src/data.rs) data construct for executing queries.

### Example projects

* [BrendanBall/example-actix-web-juniper-diesel](https://github.com/BrendanBall/example-actix-web-juniper-diesel)
* [iwilsonq/rust-graphql-example](https://github.com/iwilsonq/rust-graphql-example)
* [dancespiele/listing_people_api_actix](https://github.com/dancespiele/listing_people_api_actix)

### Blog posts

* [Building Powerful GraphQL Servers with Rust](https://dev.to/open-graphql/building-powerful-graphql-servers-with-rust-3gla)
* [[Rust] juniper + diesel + actix-webでGraphQLしてみる](https://qiita.com/yagince/items/e378bbaa95e08bab7467)
