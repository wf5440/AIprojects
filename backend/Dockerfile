# 构建阶段
FROM rust:1.71 as builder

WORKDIR /usr/src/app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

# 构建应用
RUN cargo build --release

# 生产阶段
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN useradd -m -u 1000 rustuser

WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /usr/src/app/target/release/rust-backend /app/rust-backend
COPY --from=builder /usr/src/app/migrations /app/migrations

# 设置权限
RUN chown -R rustuser:rustuser /app

USER rustuser

EXPOSE 8080

CMD ["/app/rust-backend"]