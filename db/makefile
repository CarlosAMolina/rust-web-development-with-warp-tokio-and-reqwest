CONTAINER_DATA_PATHNAME=/home/postgres/data
CONTAINER_NAME=rust-web-postgres-container
CONTAINER_VOLUME=db_postgres_rust_web
DB_NAME=rustwebdev
IMAGE_NAME=postgres
IMAGE_VERSION=15.2
POSTGRES_USER=postgres
POSTGRES_PW=pw

pull-image:
	docker pull $(IMAGE_NAME):$(IMAGE_VERSION)

create-volume:
	docker volume create $(CONTAINER_VOLUME)

run:
	# PGDATA: tells PostgreSQL where it should store the database.
	docker run \
		--rm \
		-d \
		--name $(CONTAINER_NAME) \
		-p 5432:5432 \
		-v $(CONTAINER_VOLUME):$(CONTAINER_DATA_PATHNAME) \
		-e POSTGRES_PASSWORD=$(POSTGRES_PW) \
		-e PGDATA=$(CONTAINER_DATA_PATHNAME) \
		$(IMAGE_NAME)

connect:
	docker exec -it $(CONTAINER_NAME) /bin/bash

connect-psql:
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER)

connect-psql-db:
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER) rustwebdev

stop:
	docker stop $(CONTAINER_NAME)

run-migrations:
	sqlx migrate run --database-url postgresql://$(POSTGRES_USER):$(POSTGRES_PW)@localhost:5432/$(DB_NAME)

revert-migrations:
	# Each revert works on the latests migration.
	sqlx migrate revert --database-url postgresql://$(POSTGRES_USER):$(POSTGRES_PW)@localhost:5432/$(DB_NAME)
	sqlx migrate revert --database-url postgresql://$(POSTGRES_USER):$(POSTGRES_PW)@localhost:5432/$(DB_NAME)

