defmodule RustPrimeCheck do
  use Rustler, otp_app: :elixir_nif, crate: "primecheck"

  # When your NIF is loaded, it will override this function.
  def is_prime(_number), do: :erlang.nif_error(:nif_not_loaded)
end
