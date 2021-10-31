## DeGauss
Your friendly neighborhood Avro schema compatibility checker.

## Install

```
cargo install degauss
```


## Run

```
cargo run -- -s tests/data/movies-raw-reader.avsc tests/data/movies-raw-writer.avsc -c full-transitive --exit-status
```