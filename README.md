# Rust REST API with JWT Authentication

A production-ready REST API built with Rust, Axum, PostgreSQL, and JWT authentication.

## 🚀 Features

- **JWT Authentication** - Secure token-based authentication
- **Password Hashing** - bcrypt for secure password storage
- **PostgreSQL Database** - With SQLx for type-safe queries
- **Full CRUD Operations** - Create, Read, Update, Delete posts
- **User Authorization** - Users can only access their own posts
- **CORS Enabled** - Ready for frontend integration
- **Error Handling** - Proper error responses with status codes
- **Logging** - Structured logging with tracing

## 🛠️ Tech Stack

- **Framework:** [Axum](https://github.com/tokio-rs/axum)
- **Database:** [PostgreSQL](https://www.postgresql.org/)
- **ORM/Query:** [SQLx](https://github.com/launchbadge/sqlx)
- **Authentication:** [JWT](https://github.com/Keats/jsonwebtoken) + [bcrypt](https://github.com/Keats/rust-bcrypt)
- **Deployment:** [Shuttle](https://www.shuttle.rs/)
- **Runtime:** [Tokio](https://tokio.rs/)

## 📋 API Endpoints

### Public Routes

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/api/auth/register` | Register a new user |
| `POST` | `/api/auth/login` | Login and get JWT token |

### Protected Routes (Requires JWT Token)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/posts` | Get all posts for authenticated user |
| `POST` | `/api/posts` | Create a new post |
| `GET` | `/api/posts/:id` | Get a single post |
| `PUT` | `/api/posts/:id` | Update a post |
| `DELETE` | `/api/posts/:id` | Delete a post |

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (1.75+)
- [PostgreSQL](https://www.postgresql.org/) (15+)
- [Docker](https://www.docker.com/) (optional, for PostgreSQL)

### Installation

1. **Clone the repository:**
```bash
git clone https://github.com/IgYaHiko/rust-api
cd rust-api
