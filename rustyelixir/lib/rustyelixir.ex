defmodule Rustyelixir do
  use Rustler, otp_app: :rustyelixir, crate: "rustyelixir"

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)

  def hello do
    :world
  end
end
