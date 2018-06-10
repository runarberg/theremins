FROM rust:1.26.2
LABEL maintainer="runarberg <runarberg@zoho.com>"

EXPOSE 80

WORKDIR /usr/src/theremins-http-server
ADD . .

RUN cargo install

ENV THEREMINS_HTTP_PORT=80
ENV THEREMINS_WS_URL="ws://ws.theremins.test"

CMD ["theremins-http-server"]
