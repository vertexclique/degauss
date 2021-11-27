
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

### Grab a binary from [releases](https://github.com/vertexclique/degauss/releases)

### Using cargo
```
cargo install degauss
```

## Example

- Check the compatibility of your schemas
    ```
    $ degauss validate -s tests/data/movies-raw-reader.avsc tests/data/movies-raw-writer.avsc -c full-transitive
    ```

- Check the compatibility and set the exit status in case of a failure.
    ```
    $ degauss validate -s tests/data/movies-raw-reader.avsc tests/data/movies-raw-writer.avsc -c full-transitive --exit-status
    ```

- Register a schema to schema-registry
    - create a file with env variables
    ```        
    $ cat env
    export DEGAUSS_SCHEMA_REGISTRY_URL=https://some-url
    export DEGAUSS_SCHEMA_REGISTRY_USER=some-user
    export DEGAUSS_SCHEMA_REGISTRY_PASS=some-pass
    ```
    ```
    $ source env
    ```

    ```
    $ degauss schema-registry register --subject-type value --topic test2 --schema-path ./tests/data/movies-raw-reader.avsc
    ```

- Get the compatibility for a subject:
    ```
    $ degauss schema-registry compatibility get --subject-type value --topic test
    ```

- Set the compatibility for a subject:
    ```
    $ degauss schema-registry compatibility set --subject-type value --topic test --compatibility forward
    ```
