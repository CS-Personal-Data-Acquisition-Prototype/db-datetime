DOCKERRUN = docker run --rm -t
IMAGENAME = pda-db/devcontainer
VOLUME = --volume ./crate:/data
USER = --build-arg USER=$$(whoami) \
	--build-arg UID=$$(id -u) \
	--build-arg GID=$$(id -g)

run: .image
	$(DOCKERRUN) $(VOLUME) $(IMAGENAME) /bin/bash -c "cargo run"

build: .image
	$(DOCKERRUN) $(VOLUME) $(IMAGENAME) /bin/bash -c "cargo build"

.image: Dockerfile
	docker build $(USER) --tag $(IMAGENAME) . &&\
	touch .image

.phony: cli clean

cli: .image
	$(DOCKERRUN) -i $(VOLUME) $(IMAGENAME) /bin/bash

clean:
	rm -rfv ./crate/target .image