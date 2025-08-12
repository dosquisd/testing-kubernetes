# Testing Kubernetes

Learning Kubernetes with real project applications, implementing important manifests and exploring new technologies along the way.

## Overview

This project is my playground for learning Kubernetes deployment patterns using Minikube locally. The main goal is to deploy a complete application stack in Kubernetes and use Ingress to route traffic between different API versions.

## Architecture

The project consists of multiple API server versions:

- **Server v1**: Python FastAPI application with PostgreSQL, Redis, and QuestDB
- **Server v2**: Rust Actix-web application with SeaORM migrations
- **Server v3**: (Planned) Node.js application

## Tech Stack

### Server v1 (Python)

- FastAPI
- SQLAlchemy
- PostgreSQL
- Redis
- QuestDB
- Pytest for testing

### Server v2 (Rust)

- Actix-web
- SeaORM with migrations
- PostgreSQL
- Redis
- QuestDB

### Infrastructure

- Kubernetes with Minikube
- Ingress for traffic routing
- Grafana for monitoring
- Moonrepo for monorepo management

## Project Structure

```text
apps/
├── server-v1/          # Python FastAPI application
└── server-v2/          # Rust Actix-web application
k8s/
├── development/        # Kubernetes manifests for dev environment
└── create-namespaces.yaml
scripts/
└── deploy-k8s.sh      # Deployment automation
```

## Getting Started

### Prerequisites

- Minikube
- kubectl
- Docker
- Moon (for monorepo management)

### Deployment

1. Start Minikube:

   ```bash
   minikube start
   ```

1. Enable ingress:

    ```bash
    minikube addons enable ingress
    ```

1. Deploy to Kubernetes:

   ```bash
   ./scripts/deploy-k8s.sh
   ```

1. Access the applications through Ingress routes

## Current Learning Goals

- [x] Kubernetes fundamentals and manifest creation
- [x] Ingress configuration for traffic routing
- [x] Monorepo management with Moonrepo
- [x] Rust API development with Actix-web and SeaORM
- [ ] CI/CD pipeline with GitHub Actions
- [ ] Node.js Express API (v3)
- [ ] Advanced monitoring and observability

## Notes

This is a personal learning project focused on practical Kubernetes experience and exploring new technologies. The setup is designed for local development and experimentation.
