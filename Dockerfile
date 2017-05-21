FROM liuchong/rustup

WORKDIR /app

ADD . /app

EXPOSE 8080

ENV NAME docker_image

ENV DB_USER docker
ENV DB_PASS docker
ENV DB_NAME docker
ENV LISTENING_PORT 0.0.0.0:8080
ENV DATABASE_URL postgres://docker:docker@0.0.0.0:5432/docker

RUN rustc -V
RUN cargo -V
RUN cargo build --release

CMD cargo run --release