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