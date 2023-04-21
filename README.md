# Rustfuck - A Brainfuck interpreter written in Rust

This a tiny project, whose goal was to create a simple BrainFuck interpreter. The interpreter works in two simple steps: loops mapping and interpreting. Loop mapping is used to do jumps while interpreting the code.

## Arguments

Description of interpreter arguments, to modify the interpreter output.

| Name | Description |
| - | - |
| **--hex** | Prints the output as hex numbers |
| **--bprint** | Prints the odd outputs as bold |

## Example

Here is an example of a `HelloWorld.b` execution:

```sh
cargo run -- --bprint ./test/HelloWorld.b
```

This output uses the better print, what should print the output with the odd outputs as bold:

```
Hello, World!
```
