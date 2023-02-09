FROM rust:1.66-alpine3.17

# Create app directory
WORKDIR /usr/src/hermod
# Install app dependencies
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build -r

CMD ["cargo", "run", "-r"]