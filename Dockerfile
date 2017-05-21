FROM ubuntu
FROM postgres
FROM
WORKDIR /app

ADD . /app
ADD init.sql /docker-entrypoint-initdb.d/

VOLUME ["/app/data"]

EXPOSE 8080

ENV NAME docker_image
ENV POSTGRES_USER docker
ENV POSTGRES_PASSWORD docker
ENV POSTGRES_DB docker

RUN rustc -V
RUN cargo -V
RUN cargo build

CMD cargo run