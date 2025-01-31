FROM rust:1.84.0-alpine

RUN apk update && apk add libc-dev bash

VOLUME [ "/data" ]

WORKDIR /data

ARG USER=runUser
ARG UID=1001
ARG GID=1001

RUN addgroup -g ${GID} ${USER}
RUN adduser -H -D -s /bin/sh -G ${USER} -u ${UID} ${USER}

USER ${UID}:${GID}

CMD [ "/bin/sh" ]