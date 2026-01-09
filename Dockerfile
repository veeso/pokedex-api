FROM rust:1.92

WORKDIR /usr/src/app

COPY . .
RUN cargo install --locked --path .

EXPOSE 5000

CMD ["pokedex-api", "--bind-address", "0.0.0.0", "--port", "5000", "--log-level", "info"]