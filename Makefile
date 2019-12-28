CARGO = cargo

build:
	$(CARGO) build

fmt:
	$(CARGO) fmt

open:
	open http://localhost:4000/graphql

release:
	$(CARGO) build --release

run:
	$(CARGO) run
