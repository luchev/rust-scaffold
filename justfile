mysql:
    docker run --name mysql -e MYSQL_ROOT_PASSWORD=mysql -d mysql

rm-mysql:
    docker rm -f mysql || true

restart-mysql: rm-mysql mysql

connect-mysql:
    docker exec -it mysqe mysql -uroot -pmysql

ping:
    grpcurl \
    -plaintext \
    -import-path proto \
    -proto app.proto \
    -d "{}" \
    "[::1]:2000" \
    app_grpc.AppService/Ping

ping-remote:
    grpcurl \
    -plaintext \
    -import-path proto \
    -proto app.proto \
    -d "{\"peer\": \"QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N\"}" \
    "[::1]:2000" \
    app_grpc.AppService/PingRemote


run env:
    ENV={{env}} RUST_LOG=info cargo run

killall:
    killall just
