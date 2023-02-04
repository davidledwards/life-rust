# Conway's Game of Life

A Rust implementation of the original version of Conway's Game of Life [written in Scala](https://github.com/davidledwards/life).

The current implementation is not meant to be the most efficient algorithm but rather a somewhat literal interpretation of the functional style used in the Scala version.

## Instructions

Install [Rust](https://www.rust-lang.org/tools/install) if not done so already.

In the root directory of `life-rust`, run either of the following commands to build the project.

```shell
$ cargo build     # debug build
$ cargo build -r  # release build
```

Running the program can be done via `cargo` or directly using the target executable.

```shell
$ cargo run [-r] -- [options to life go here]
```

```shell
$ target/debug/life-rust [options]
$ target/release/life-rust [options]
```

If the program is executed with no arguments, it will run for an infinite number of generations with a delay of 100 milliseconds before advancing to the next generation.

Use `--help` to print a description of all available options.

## License

Copyright 2023 David Edwards

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

<http://www.apache.org/licenses/LICENSE-2.0>

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
