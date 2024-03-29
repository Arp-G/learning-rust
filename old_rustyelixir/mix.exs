defmodule Rustyelixir.MixProject do
  use Mix.Project

  def project do
    [
      app: :rustyelixir,
      version: "0.1.0",
      elixir: "~> 1.10",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: Mix.compilers(),
      rustler_crates: rustler_crates()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.26.0"}
    ]
  end

  defp rustler_crates do
    [
      rusty: [
        path: "native/rustyelixir",
        mode: :release#rustc_mode(Mix.env())
      ]
    ]
  end

  defp rustc_mode(:prod), do: :release
  defp rustc_mode(_), do: :debug
end
