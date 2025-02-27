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

| Name                    | Description                                                                 |
|-------------------------|-----------------------------------------------------------------------------|
| `RUST_LOG`              | Logging level for tracing (`debug`, `info`, `error`).                       |
| `SERVICE`               | Package name to be executed. Default is `main-ui`.                          |
| `API_URL`               | Base URL for the backend API. Used by the frontend to interact with the API.|
| `ENV`                   | Environment type (dev, prod, etc.).                                         |
| `KLAYTN_ENDPOINT`       | Klaytn blockchain endpoint.                                                 |
| `AWS_ACCESS_KEY_ID`     | AWS access key ID.                                                          |
| `AWS_SECRET_ACCESS_KEY` | AWS secret access key.                                                      |

## Development

### Prerequisites
Before you start, ensure you have the following installed:

1. **Rust**: Install it from [rustup.rs](https://rustup.rs/).
2. **PostgreSQL**: Set up a PostgreSQL database and update the connection URL in the configuration.
4. **Node.js** (for frontend dependencies)
5. **Docker** (optional, for containerized builds)
6. **AWS CLI** (for AWS-related configurations)
7. **cargo-binstall** (for installing Rust binaries)

### Setting Up the Project
1. Clone the Repository:
```bash 
git clone https://github.com/your-repo/incheon-heroes.git
cd incheon-heroes
```
2. Initialize Submodules:
```bash 
git submodule update --init 
```

### Running API Server (main-api)

The API server handles RESTful operations, database migrations, and integrations with blockchain services (e.g., Klaytn).

```bash
cd packages/main-api
make run
```

### Running Web UI (main-ui)

The frontend communicates with the API server and renders dynamic content for users.

```bash
cd packages/main-ui
make setup.tool
make run
```

### Additional Configuration
- AWS Configuration: The project uses AWS for asset storage and other services. 
  Ensure your AWS credentials are correctly set up.

- Klaytn Configuration: The project integrates with the Klaytn blockchain for certain functionalities. 
  Ensure you have the correct Klaytn endpoint and contract addresses configured.

- Database Configuration: The main-api includes a migration script that sets up the necessary database tables. 
  Ensure your database is properly configured before running the API.

### Contributing

Contributions are welcome! Please follow the standard fork-and-pull request workflow.