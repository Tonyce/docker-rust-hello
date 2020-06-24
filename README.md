# macOS 构建并使用 Docker（debian） 运行 Rust 程序

## 安装跨平台工具链

https://hackernoon.com/cross-compiling-rust-on-macos-to-run-as-a-unikernel-ff1w3ypi

brew install FiloSottile/musl-cross/musl-cross

rustup target add x86_64-unknown-linux-musl

## 新建项目：

```bash
cargo new hello-world
```

在项目目录下添加 .cargo/config 文件，并写入

```conf
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

```bash
TARGET_CC=x86_64-linux-musl-gcc \
RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" \
cargo build --target=x86_64-unknown-linux-musl
```

```bash
docker build -t rust/hello-world .

docker run --name test-rust-hello-world -it --rm rust/hello-world
Hello, world!
```

## 在 build 时传变量

```bash
ARG RUST_ENV=alpha
ENV RUST_ENV=$RUST_ENV

docker build --build-arg RUST_ENV=beta -t rust/hello-world:v1.0.0 .
```

## 使用 docker

```bash
> docker run -P -d --rm rust/hello-world:v1.0.0

> docker container ls
CONTAINER ID        IMAGE                     COMMAND                  CREATED             STATUS              PORTS                     NAMES
b26674179e4d        rust/hello-world:v1.0.0   "/app/hello-world"       19 seconds ago      Up 26 seconds       0.0.0.0:32773->8080/tcp   admiring_gates
19b9563c52f4        node/hello-world          "docker-entrypoint.s…"   12 minutes ago      Up 12 minutes       0.0.0.0:32772->8989/tcp   mystifying_faraday

> docker-machine ip
192.168.99.101

> curl -i 192.168.99.101:32773
HTTP/1.1 200 OK
content-length: 13
date: Wed, 24 Jun 2020 08:24:58 GMT
content-type: text/plain;charset=utf-8

Hello, world!%
```

## TODO

stats 监控

1.每个 container 上报 ？
2.docker stats
