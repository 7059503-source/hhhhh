# 使用官方Rust镜像作为构建阶段
FROM rust:latest AS builder

# 设置工作目录
WORKDIR /app

# 安装trunk（用于构建前端）
RUN cargo install trunk wasm-bindgen-cli

# 复制Cargo.toml和源代码
COPY Cargo.toml ./
COPY backend/ ./backend/
COPY frontend/ ./frontend/

# 构建前端
WORKDIR /app/frontend
RUN trunk build --release

# 构建后端
WORKDIR /app/backend
RUN cargo build --release

# 使用更小的镜像作为运行阶段
FROM debian:bookworm-slim

# 安装必要的依赖
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制构建好的后端
COPY --from=builder /app/backend/target/release/backend /app/

# 复制构建好的前端
COPY --from=builder /app/frontend/dist /app/frontend/dist

# 创建数据库目录
RUN mkdir -p /data

# 设置环境变量
ENV PORT=3000

# 暴露端口
EXPOSE 3000

# 运行应用
CMD ["./backend"]