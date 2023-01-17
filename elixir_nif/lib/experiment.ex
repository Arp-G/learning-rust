defmodule Experiment do
  def call do
    # huge_primes = [1_299_791, 32_452_867, 49_979_687, 452_930_477, 472_882_027]

    # huge_primes = [961_748_941, 982_451_653, 452_930_477, 472_882_027]
    huge_primes = [961_748_941]

    test_elixir_func = fn ->
      huge_primes
      |> Enum.map(fn num ->
        result = ElixirPrimeCheck.is_prime(num)
        IO.puts("#{num} is Prime: #{result}")
      end)
    end

    test_rust_func = fn ->
      huge_primes
      |> Enum.map(fn num ->
        result = RustPrimeCheck.is_prime(num)
        IO.puts("#{num} is Prime: #{result}")
        result
      end)
    end

    IO.puts("FINDING PRIME NUMBERS IN ELIXIR:")
    IO.puts("")
    measure(test_elixir_func)
    IO.puts("")
    IO.puts("-------------------------------")
    IO.puts("FINDING PRIME NUMBERS USING RUST NIF:")
    IO.puts("")
    measure(test_rust_func)
    IO.puts("")
  end

  def measure(func) do
    time =
      func
      |> :timer.tc()
      |> elem(0)
      |> Kernel./(1_000_000)

    "#{IO.ANSI.bright()} Operation took: #{time} seconds #{IO.ANSI.bright()}"
    |> IO.ANSI.format()
    |> IO.puts()
  end
end
