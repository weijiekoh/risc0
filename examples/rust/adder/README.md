# A Hello World example on Risc0

Uses Risc0 to create and verify simple addition in zero-knowledge. Allows a
prover to show that they have correctly added two 32-bit numbers in the RISC0
VM, without revealing the result (even though anyone can do the addition
themselves).


## Running the example

```console
$ bazelisk run //examples/rust/adder:adder -- --a 1 --b 2
...
[2022-05-06T08:21:47Z INFO  adder] Value A: 1
[2022-05-06T08:21:47Z INFO  adder] Value B: 2
```

## Running the tests

Using [Bazelisk](https://github.com/bazelbuild/bazelisk), simply run

```console
$ RISC0_LOG=1 bazelisk run //examples/rust/adder:test
`
