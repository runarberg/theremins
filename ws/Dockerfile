# Teremins version 0.1.0

FROM schickling/rust

MAINTAINER Rúnar Berg "runarberg@zoho.com"

ADD . /opt/theremins/

RUN cd /opt/theremins/server && \
    cargo build --release && \
    ln -s /opt/theremins/server/target/release/theremins-server \
       /usr/local/bin/theremins

CMD ["theremins", "--http-server", "0.0.0.0:80", "--ws-server", "0.0.0.0:81", "--ws-url", "theremins.club"]

EXPOSE 80 81
