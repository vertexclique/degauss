
<div align="center">
 <p><h1>DeGauss</h1> </p>
  <p><strong>Your friendly neighborhood Avro schema compatibility checker.</strong> </p>
<p>

[![cicd](https://github.com/vertexclique/degauss/actions/workflows/cicd.yml/badge.svg)](https://github.com/vertexclique/degauss/actions/workflows/cicd.yml)
[![Crates.io](https://img.shields.io/crates/v/degauss)](https://crates.io/crates/degauss)
[![Docs.rs](https://docs.rs/degauss/badge.svg)](https://docs.rs/degauss)
[![codecov](https://codecov.io/gh/vertexclique/degauss/branch/master/graph/badge.svg)](https://codecov.io/gh/vertexclique/degauss)
</p>
</div>
</br>


## Install

```
cargo install degauss
```

## Example

- Check the compatibility of your schemas
    ```
    degauss -s tests/data/movies-raw-reader.avsc tests/data/movies-raw-writer.avsc -c full-transitive
    ```

- Check the compatibility and set the exit status in case of a failure.
    ```
    degauss -s tests/data/movies-raw-reader.avsc tests/data/movies-raw-writer.avsc -c full-transitive --exit-status
    ```