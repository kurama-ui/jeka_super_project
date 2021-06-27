FROM rust:latest
WORKDIR /jeka_super_project
COPY . .
RUN cargo build
CMD ["cargo", "run"]
