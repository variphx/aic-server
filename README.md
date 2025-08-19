# 🚀 AIC Server

A high-performance backend service built with **Rust**.  
Easily deployable with **Cargo** or **Docker**, with a built-in Swagger UI for API exploration.

---

## 📦 Build & Run

### 🔨 Option 1: Build with Cargo

```bash
cargo build --release
./target/release/aic-server
```

### 🐳 Option 2: Build with Docker

```bash
docker build -t aic-server .
docker run -p 8080:8080 -e DATABASE_URL="your_database_url_here" aic-server
```

---

## ⚙️ Environment Variables

Before running, make sure to set:

```bash
export DATABASE_URL="postgres://user:password@localhost:5432/db_name"
```

This is required for database connectivity.

---

## 📖 API Documentation

After starting the server, you can explore the API via **Swagger UI**:

👉 [http://localhost:8080/swagger-ui](http://localhost:8080/swagger-ui)

---

## 🗂️ Project Structure

```text
AIC Server/
├── src/              # Source code
├── Cargo.toml        # Rust dependencies
├── Dockerfile        # Container build file
└── README.md         # You're here
```

---

## ✅ Features

- ⚡ Fast & memory-safe (built with Rust)
- 🐳 Dockerized for easy deployment
- 📚 Interactive Swagger UI
- 🗄️ PostgreSQL database support

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!
