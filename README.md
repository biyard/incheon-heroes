# Incheon Heroes

Incheon Heroes is a full-stack application built with Rust. It consists of a backend API (`main-api`) and a frontend UI (`main-ui`). The backend handles database operations, user authentication, and integrations with external services like **PostgreSQL**, **Klaytn blockchain** and **AWS**. The frontend provides a user-friendly interface for interacting with the backend.

## Project Structure

- **`main-api`**: Backend API built with `axum`.
  - Handles database migrations, user authentication, and external integrations.
  - Uses PostgreSQL for data storage.
  - Integrates with Klaytn and AWS for blockchain and asset management.

- **`main-ui`**: Frontend UI built with Dioxus.
  - Provides a responsive and interactive user interface.
  - Uses Tailwind CSS and DaisyUI for styling.
  - Communicates with the backend API for data fetching and updates.

## Common Environments

| Name       | Description                                                                 |
|------------|-----------------------------------------------------------------------------|
| `RUST_LOG` | Logging level for tracing (`debug`, `info`, `error`).                       |
| `SERVICE`  | Package name to be executed. Default is `main-ui`.                          |
| `API_URL`  | Base URL for the backend API. Used by the frontend to interact with the API.|

## Development

### Prerequisites

1. **Rust**: Ensure Rust is installed. If not, install it from [rustup.rs](https://rustup.rs/).
2. **PostgreSQL**: Set up a PostgreSQL database and update the connection URL in the configuration.
3. **Git Submodules**: Initialize submodules to fetch dependencies.
   ```bash
   git submodule update --init

### **Running API Server (main-api)**

The API server handles RESTful operations, database migrations, and integrations with blockchain services (e.g., Klaytn).

```bash
cd packages/main-api
make run
```

### **Running Web UI (main-ui)**

The frontend communicates with the API server and renders dynamic content for users.

```bash
cd packages/main-ui
make setup.tool
make run