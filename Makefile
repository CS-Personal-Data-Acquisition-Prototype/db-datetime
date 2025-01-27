IMAGENAME = pda-db/devcontainer
VOLUME = --volume ./src:/data
USER = --build-arg USER=$$(whoami) \
	--build-arg UID=$$(id -u) \
	--build-arg GID=$$(id -g)

.phony: run image clean cli

build: image
	docker run 

cli: image
	docker run -it --rm $(VOLUME) $(IMAGENAME)

image:
	docker build $(USER) --tag $(IMAGENAME) .
