# Platform HRM Rust

An enterprise-grade Human Resource Management (HRM) Microservices Platform built with **Rust**, designed for high performance, memory safety, and massive scalability.

## 🚀 Overview

This project implements a multi-tenant HRM system using a modern Microservices architecture. It demonstrates advanced backend engineering patterns in Rust, including gRPC-Gateway with protocol multiplexing, Domain-Driven Design (DDD), and centralized Identity and Access Management (IAM).

## 🏗️ Architecture

The system is composed of several specialized microservices communicating via **gRPC** for internal calls and providing a unified **REST API** through the API Gateway.

### 1. API Gateway (The Brain)
*   **Protocol Multiplexing:** Utilizes a custom `Tower` service to handle both HTTP/REST and gRPC traffic on a single port (Default: 8000).
*   **gRPC-Gateway:** Acts as a transcoder, mapping RESTful JSON requests to internal gRPC Protobuf calls.
*   **Auth Integration:** Centralized JWT validation using Keycloak JWKS.
*   **Features:** Rate limiting, Request ID propagation, Compression, and Observability (Tracing/Metrics).

### 2. Microservices
*   **Auth Service:** Identity management, Keycloak integration, and fine-grained RBAC (Role-Based Access Control).
*   **Platform Service:** Multi-tenancy management, tenant-specific database provisioning, and platform-level configurations.
*   **HRIS Service:** Core HR functions including Employee Lifecycle, Payroll Processing, Attendance Tracking, and Leave Management.

## 🛠️ Technical Stack

*   **Language:** [Rust](https://www.rust-lang.org/) (Edition 2021)
*   **Web Framework:** [Axum](https://github.com/tokio-rs/axum) (HTTP/REST)
*   **gRPC Framework:** [Tonic](https://github.com/hyperium/tonic) (Protobuf)
*   **Async Runtime:** [Tokio](https://tokio.rs/)
*   **IAM:** [Keycloak](https://www.keycloak.org/) (OIDC/JWT)
*   **Serialization:** [Serde](https://serde.rs/), [Prost](https://github.com/tokio-rs/prost)
*   **Observability:** [Tracing](https://github.com/tokio-rs/tracing), [Metrics-exporter-prometheus](https://github.com/metrics-rs/metrics)
*   **Infrastructure:** Docker, Docker Compose, PostgreSQL

## 🌟 Key Features Demonstrated

*   **Distributed Systems:** Service discovery and inter-service communication via gRPC.
*   **Memory Safety & Performance:** Leveraging Rust's ownership model to ensure a crash-free environment with C/C++ level speed.
*   **Clean Architecture:** Strict separation of concerns (Domain, Application, Infrastructure, Interface).
*   **Multi-tenancy:** Isolated data structures for different organizations on the same platform.
*   **Advanced Networking:** Port-sharing between REST and gRPC protocols.

## 🚦 Getting Started

### Prerequisites
*   Rust (Latest Stable)
*   Docker & Docker Compose
*   `grpcurl` (Optional, for testing gRPC)

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/danai-gingpho/platform-hrm-rust.git
   cd platform-hrm-rust
   ```
2. Set up environments:
   ```bash
   cp .env.example .env
   # Adjust config.toml in api-gateway and other services
   ```
3. Run with Docker Compose:
   ```bash
   docker-compose up -d
   ```

## 🧪 Testing the Gateway
**REST API:**
```bash
curl http://localhost:8000/health
```

**gRPC Reflection:**
```bash
grpcurl -plaintext localhost:8000 list
```

---
Developed with ❤️ by [Danai Gingpho](https://github.com/danai-gingpho)
