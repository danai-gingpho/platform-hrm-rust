.DEFAULT_GOAL := help
SHELL          := /bin/bash
CARGO          ?= cargo
DOCKER_COMPOSE ?= docker compose
PORT           ?= 8000
HEALTH_URL     ?= http://localhost:$(PORT)/health

.PHONY: help
help: ## Show this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n\nTargets:\n"} \
		/^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

.PHONY: env
env: ## Copy .env.example -> .env and config.example.toml -> config.toml (no overwrite)
	@test -f .env || cp .env.example .env
	@test -f config.toml || cp config.example.toml config.toml
	@echo ".env and config.toml ready"

.PHONY: build
build: ## cargo build
	$(CARGO) build

.PHONY: release
release: ## cargo build --release
	$(CARGO) build --release

.PHONY: run
run: ## Run the gateway locally
	$(CARGO) run

.PHONY: check
check: ## cargo check
	$(CARGO) check --all-targets

.PHONY: fmt
fmt: ## cargo fmt
	$(CARGO) fmt --all

.PHONY: lint
lint: ## cargo clippy -D warnings
	$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: test
test: ## cargo test
	$(CARGO) test

.PHONY: ci
ci: lint test ## Lint + test

.PHONY: up
up: ## docker compose up --build (gateway + keycloak)
	$(DOCKER_COMPOSE) up -d --build

.PHONY: down
down: ## docker compose down
	$(DOCKER_COMPOSE) down

.PHONY: logs
logs: ## Tail gateway logs
	$(DOCKER_COMPOSE) logs -f api-gateway

.PHONY: kc-logs
kc-logs: ## Tail keycloak logs
	$(DOCKER_COMPOSE) logs -f keycloak

.PHONY: health
health: ## Curl /health
	@curl -fsS $(HEALTH_URL) && echo

.PHONY: metrics
metrics: ## Curl /metrics
	@curl -fsS http://localhost:$(PORT)/metrics

.PHONY: tidy
tidy: fmt lint ## Format and lint code

.PHONY: clean
clean: ## cargo clean
	$(CARGO) clean

.PHONY: setup
setup: ## Install development tools (clippy, rustfmt)
	rustup component add clippy rustfmt