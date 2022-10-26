# latest stable release at time of writing
FROM rust:1.64.0

# creates working directory app if not already created
WORKDIR /app

#installs required system dependencies for linking configuration
RUN apt update && apt install lld clang -y

# copy all files from working environment to Docker image
COPY . .

# build release profile for better performance
RUN cargo build --release

# forces docker to read offline metadata instead of trying to query a live database
ENV SQLX_OFFLINE true

# launches release binary when docker run is executed
ENTRYPOINT ["./target/release/zero2prod"]