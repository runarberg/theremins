FROM rust:1.26.2
LABEL maintainer="runarberg <runarberg@zoho.com>"

EXPOSE 80

WORKDIR /usr/src/theremins-ws-server
ADD . .

RUN cargo install

ENV THEREMINS_WS_PORT=80

CMD ["theremins-ws-server"]
