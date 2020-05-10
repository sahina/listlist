FROM debian:bullseye-slim
WORKDIR /app
ADD target/release/listlist .
CMD ["/app/listlist"]