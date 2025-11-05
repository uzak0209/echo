# Echo Backend

Rust + GraphQL + SeaORM backend for the Echo SNS project.

## Setup

1. Install Rust: https://rustup.rs/

2. Install PostgreSQL and create a database:
```bash
createdb echo
```

3. Copy `.env.example` to `.env` and configure your database URL:
```bash
cp .env.example .env
```

4. Run migrations:
```bash
cargo install sea-orm-cli
sea-orm-cli migrate up -d migration
```

5. Run the server:
```bash
cargo run
```

The GraphQL Playground will be available at http://localhost:8000

## GraphQL Schema

### Queries

```graphql
query {
  timeline(limit: 10) {
    id
    content
    imageUrl
  }
}
```

### Mutations

```graphql
mutation {
  createPost(content: "Hello, Echo!", imageUrl: null)
}

mutation {
  incrementDisplayCount(postId: 1)
}
```

## Development

Run in watch mode:
```bash
cargo install cargo-watch
cargo watch -x run
```
