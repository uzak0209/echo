.PHONY: help up down restart build logs migrate migrate-fresh clean test

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

up: ## Start all services
	docker compose up -d

down: ## Stop all services
	docker compose down

restart: ## Restart all services
	docker compose restart

build: ## Build all services
	docker compose up --build -d

logs: ## Show logs (use: make logs SERVICE=backend)
	@if [ -z "$(SERVICE)" ]; then \
		docker compose logs -f; \
	else \
		docker compose logs -f $(SERVICE); \
	fi

migrate: ## Run database migrations
	docker compose exec backend bash -c "cd migration && cargo run -- up"

migrate-fresh: ## Drop all tables and re-run migrations
	docker compose exec backend bash -c "cd migration && cargo run -- fresh"

migrate-reset: ## Rollback all migrations and re-run them
	docker compose exec backend bash -c "cd migration && cargo run -- reset"

seed: ## Insert seed data into database
	@echo "Inserting seed data..."
	docker compose exec -T postgres psql -U postgres -d echo < backend/seed.sql
	@echo "✓ Seed data inserted successfully!"

seed-reset: migrate-fresh seed ## Reset database and insert seed data
	@echo "✓ Database reset and seeded!"

clean: ## Remove all containers, volumes, and images
	docker compose down -v
	docker system prune -f

test: ## Run tests
	docker compose exec backend cargo test

shell-backend: ## Open shell in backend container
	docker compose exec backend bash

shell-frontend: ## Open shell in frontend container
	docker compose exec frontend sh

db: ## Connect to PostgreSQL database
	docker compose exec postgres psql -U postgres -d echo

# Development shortcuts
dev: up logs ## Start services and show logs

rebuild: down build ## Rebuild and restart all services
