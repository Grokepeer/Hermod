FROM rust:1.66-alpine3.17

# Create app directory
WORKDIR /usr/src/hermod
# Install app dependencies
COPY ./src ./src
COPY ./cargo.toml ./Cargo.toml
COPY ./.cargo ./.cargo

RUN cargo build -r

CMD ["cargo", "run", "-r"]