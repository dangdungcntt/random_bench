# random_bench

Simple benchmarks for random number generation in Bun, Node.js, Go and PHP.

## Prerequisites

Bun, Node.js, Go and PHP

## Run

```bash
./run.sh
```

## Results

```
[Bun 1.0.18]
Total run: 100,000,000, elapsed: 1.31s. 13.12 ns/run, 76,219,512 runs/s
id: 0, count: 10000426 (10.000426%)
id: 1, count: 19999046 (19.999046%)
id: 2, count: 29999153 (29.999153%)
id: 3, count: 40001375 (40.001375%)

[go version go1.21.4 darwin/arm64]
Total run: 100,000,000, elapsed: 1.55s. 15.00 ns/run, 64,474,000 runs/s
id: 0 count: 10003862 (10.003862%)
id: 1 count: 19995431 (19.995431%)
id: 2 count: 30005358 (30.005358%)
id: 3 count: 39995349 (39.995349%)

[NodeJS v20.10.0]
Total run: 100,000,000, elapsed: 2.05s. 20.54 ns/run, 48,685,491 runs/s
id: 0, count: 10000456 (10.000456%)
id: 1, count: 19998012 (19.998012%)
id: 2, count: 30004250 (30.004250%)
id: 3, count: 39997282 (39.997282%)

[PHP 8.1.22 (cli) (built: Aug  9 2023 07:54:46) (NTS)]
Total run: 100,000,000, elapsed: 9.59s. 95.91 ns/run, 10,425,923 runs/s
id: 0, count: 10001278 (10.00%)
id: 1, count: 19998035 (20.00%)
id: 2, count: 29998031 (30.00%)
id: 3, count: 40002656 (40.00%)
```