
.PHONY: migrate
migrate:
	sea-orm-cli migrate up
	$(MAKE) entities

.PHONY: migrate/fresh
migrate/fresh:
	sea-orm-cli migrate fresh
	$(MAKE) entities

.PHONY: entities
entities:
	rm ./entity/src/entities/*.rs
	sea-orm-cli generate entity -o ./entity/src/entities
