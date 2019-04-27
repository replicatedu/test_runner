/root/.cargo/bin/cargo build --release --target=x86_64-unknown-linux-musl
docker build .
docker push hortinstein/replicatedu_tester:demo_time