defmodule ElixirNifTest do
  use ExUnit.Case
  doctest ElixirNif

  test "greets the world" do
    assert ElixirNif.hello() == :world
  end
end
