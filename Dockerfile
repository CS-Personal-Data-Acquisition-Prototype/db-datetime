FROM archlinux/archlinux:latest
LABEL Name="PDA-DB-MANAGEMENT-DevContainer"

VOLUME [ "/data" ]

RUN pacman -Syu
RUN pacman -S rust --noconfirm

ARG USER=runUser
ARG UID=1001
ARG GID=1001

RUN groupadd -g ${GID} -o ${USER}
RUN useradd -u ${UID} -g ${GID} -s /bin/bash ${USER}

USER ${UID}:${GID}
CMD [ "/bin/bash" ]
