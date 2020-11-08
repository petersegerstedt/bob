cross build --target x86_64-unknown-linux-musl --release
cp target/x86_64-unknown-linux-musl/release/bob docker/
cp -r static docker/
cd docker
docker build -t peter/bob .
