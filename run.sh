echo "\n[Bun $(bun --version)]"
bun random_bench.js
echo "\n[$(go version)]";
go build random_bench.go && ./random_bench && rm ./random_bench
echo "\n[NodeJS $(node -v)]"
node random_bench.js
echo "\n[$(php -v | head -n 1)]"
php random_bench.php
echo "\n[$(cargo --version | head -n 1)]"
cargo run --bin random_bench --quiet --release