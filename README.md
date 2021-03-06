# nota
[![GPL 3 licensed](https://img.shields.io/badge/license-ISC-green.svg)](./LICENSE)

Note taking app based on OrbTk https://github.com/redox-os/orbtk. Currently heavily based on PluralPlanner. This will change definitely over time. But it's a first starting point.

## Platforms

* Redox OS (native)
* Linux (native | cargo-node wip)
* macOS (native | cargo-node wip)
* Windows (native | cargo-node wip)
* openBSD (not tested, but should work)
* Web

## Run 

You can start the app by executing the following command:

```text
cargo run --release
```

To start the app with the light theme execute following command:

```shell
cargo run --release --features light
```

## Run with cargo-node

To run the editor on as browser or electron app you have to install

```text
cargo install -f cargo-node
```

Before you could use cargo node you have to install `npm` version 6.9.0. It is included in the `Node.js` version 10.16.3. You could download it from https://nodejs.org/dist/v10.16.3/. 

Rust's `cargo` is presumed. All other dependencies of cargo node will be installed automatic.

### Start 

* Run as browser app:

```text
cargo node run --browser
```

* Run as electron app:

```text
cargo node run --electron
```

## License

Licensed under ISC license ([LICENSE](LICENSE))
