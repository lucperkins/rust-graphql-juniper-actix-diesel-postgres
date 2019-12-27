CARGO = cargo

build:
	$(CARGO) build

fmt:
	$(CARGO) fmt

open:
	open http://localhost:8080/graphql

release:
	$(CARGO) build --release

run:
	$(CARGO) run
