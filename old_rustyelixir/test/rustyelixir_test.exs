defmodule RustyelixirTest do
  use ExUnit.Case
  doctest Rustyelixir

  test "greets the world" do
    assert Rustyelixir.hello() == :world
  end
end
