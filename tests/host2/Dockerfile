FROM rust:1-buster

WORKDIR /ekkyo
COPY . .
RUN cargo build

CMD ["./target/debug/ekkyo", "64513 10.200.100.3 64512 10.200.100.2 passive"]
