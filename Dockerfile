# 使用 Amazon Linux 官方 Docker 镜像作为基础镜像
FROM --platform=linux/amd64 amazonlinux:latest AS builder

# 安装编译工具、Rust 和 SQLite 开发库
RUN yum update -y && \
    yum install -y gcc sqlite-devel && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# 设置工作目录
WORKDIR /usr/src/rocket-app

# 设置自定义的目标目录
ENV CARGO_TARGET_DIR=/rocket

# 复制你的 Rust 项目文件到工作目录
COPY . .

# 构建你的 Rust 应用
RUN cargo build --release

# 设置容器启动时执行的命令（假设你的应用名为 myapp）
CMD ["/rocket/release/rocket-app"]




