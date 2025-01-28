DOCKERRUN = docker run --rm
IMAGENAME = pda-db/devcontainer
VOLUME = --volume ./crate:/data
USER = --build-arg USER=$$(whoami) \
	--build-arg UID=$$(id -u) \
	--build-arg GID=$$(id -g)

.phony: run cli image clean

run:
	$(DOCKERRUN) $(VOLUME) $(IMAGENAME) /bin/sh -c "cargo run"

build:
	$(DOCKERRUN) $(VOLUME) $(IMAGENAME) /bin/sh -c "cargo build"

cli: image
	$(DOCKERRUN) $(VOLUME) $(IMAGENAME)

image: Dockerfile
	docker build $(USER) --tag $(IMAGENAME) .

clean:
	rm -rfv ./crate/target