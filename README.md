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

> Before you get started, make sure that you have Rust and Cargo installed.

```bash
git clone https://github.com/lucperkins/rust-actix-diesel-postgres-juniper
cd rust-actix-diesel-postgres-juniper
cargo run # could take a while!
```

Then you can access the GraphQL Playground UI at http://localhost:4000/graphql.

## Schema

The server implements the following schema:

```graphql
type Todo {
  id: ID!
  task: String!
  done: Boolean
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

## Future TODOs

Get it? Anyway, here's some areas for improvement (pull requests very much welcome):

* **Error handling** — Right now errors basically propagate directly from Diesel/Postgres into the GraphQL JSON output, which is subpar. If any of you can point me to good educational resources on this, please file an issue!
* **Better execution engine** — The server uses the extremely powerful [actix-web](https://github.com/actix/actix-web) but the actual DB interactions don't use Actix actors and it'd take this setup to the next level if they did.
* **Use macros for schema generation** — The powerful [`juniper_from_schema`](https://docs.rs/juniper-from-schema/0.5.1/juniper_from_schema/) macro could help reduce boilerplate and improve development velocity.
