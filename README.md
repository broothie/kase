# `kase`

Convert from one case to another. Input case is determined on a best-guess basis, but can be overridden with the --from flag.

## Installation

```shell script
cargo install kase
```

## Usage

### Example

```console
$ kase snake MyVariable
my_variable

$ kase screaming-snake my_variable
MY_VARIABLE

$ kase kebab MY_VARIABLE
my-variable

$ kase path my-variable
my/variable

$ kase dot my/variable
my.variable

$ kase camel my.variable
myVariable

$ kase pascal myVariable
MyVariable

# If the best-guess for an input isn't right for your use case, you can use the `--from` flag:
$ kase dot my_dir/my_path
my.dir/my.path

$ kase --from path dot my_dir/my_path
my_dir.my_path
```

### Options

```console
$ kase -h
Convert from one case to another. Input case is determined on a best-guess basis, but can be overridden with the --from flag.

Usage: kase [OPTIONS] <CASE> [INPUT]

Arguments:
  <CASE>   Case to convert to [possible values: snake, screaming-snake, kebab, path, dot, camel, pascal, lower]
  [INPUT]  String to convert; if empty, reads from stdin

Options:
  -f, --from <FROM>  Case to convert from, if best-guess isn't working [possible values: snake, screaming-snake, kebab, path, dot, camel, pascal, lower]
      --debug        Debug mode
  -h, --help         Print help information
  -V, --version      Print version information
```
