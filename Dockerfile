FROM supremacy037/rust-sequence-generator:latest

WORKDIR /app

ADD . /app

RUN pip install -r requirements.txt

EXPOSE 8080

ENV NAME test

RUN rustc -V
RUN cargo -V
RUN cargo build

CMD cargo run