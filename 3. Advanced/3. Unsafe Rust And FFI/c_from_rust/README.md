## Build C Library
`clang -c adder/adder.c -o adder/adder.o`
`ar -rc ./adder/libadder.a ./adder/adder.o`

## Run
`RUSTFLAGS='-L ./adder' cargo run`