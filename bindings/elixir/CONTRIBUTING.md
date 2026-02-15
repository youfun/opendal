# Contributing

Thank you for contributing to the Elixir bindings for Apache OpenDAL!

## Prerequisites

- [Elixir](https://elixir-lang.org/install.html) 1.14 or later
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

## Setup

1. Install Elixir dependencies:
    ```sh
    mix deps.get
    ```

## Testing

Run tests with `mix test`:

```sh
mix test
```

## Development

The native code is located in `native/opendal_elixir`. When you modify Rust code, `rustler` will automatically recompile it when you run `mix test` or start an application.

## Formatter

Format Elixir code:

```sh
mix format
```
