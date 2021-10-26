## DeGauss
You friendly neighborhood Kafka-Schema compatibility checker.

## Run

cargo build && ./target/debug/degauss --old tests/data/movies-raw-reader.avsc  --new tests/data/movies-raw-writer.avsc --exit-status
