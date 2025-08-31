FROM rust:trixie

COPY static static
COPY target/release/core .

EXPOSE 8080

CMD ["./core", "server"]
