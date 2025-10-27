.PHONY: docker-build docker-up docker-down docker-logs docker-clean docker-ps

DOCKER_COMPOSE ?= docker compose
PROJECT_NAME ?= hhton

## Build the runtime image for the gRPC service
docker-build:
$(DOCKER_COMPOSE) build

## Start the full local stack (service + dependencies)
docker-up:
$(DOCKER_COMPOSE) -p $(PROJECT_NAME) up -d

## Stop the local stack and keep volumes intact
docker-down:
$(DOCKER_COMPOSE) -p $(PROJECT_NAME) down

## Tail logs from all containers in the stack
docker-logs:
$(DOCKER_COMPOSE) -p $(PROJECT_NAME) logs -f

## Show running containers for the project
docker-ps:
$(DOCKER_COMPOSE) -p $(PROJECT_NAME) ps

## Tear down containers and delete named volumes
docker-clean:
$(DOCKER_COMPOSE) -p $(PROJECT_NAME) down -v
