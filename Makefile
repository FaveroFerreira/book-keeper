SHELL = /bin/bash

.DEFAULT_GOAL := help

.PHONY: help

help: ## show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


up: ## create setup local dependencies
	docker compose up -d --remove-orphans
	@sleep 5
	bash scripts/scylla/check-scylla-startup.sh
	bash scripts/scylla/initialize-scylla-db.sh

down:
	docker compose stop -t 0