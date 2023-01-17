# NIF for Elixir.PrimeCheck

## To build the NIF module:

- Your NIF will now build along with your project.
- If there are issues with "git" build the rust project via `CARGO_NET_GIT_FETCH_WITH_CLI=true cargo build`
- Then run `Experiment.call` in iex console to see it in action

## To load the NIF:

```elixir
defmodule PrimeCheck do
  use Rustler, otp_app: :elixir_nif, crate: "primecheck"

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
```

## Examples

[This](https://github.com/rusterlium/NifIo) is a complete example of a NIF written in Rust.
