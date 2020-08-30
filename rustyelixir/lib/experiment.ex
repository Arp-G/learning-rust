defmodule Experiment do
  def compare_prime_check do
    # huge_primes = [1_299_791, 32_452_867, 49_979_687, 452_930_477, 472_882_027]

    huge_primes = [961_748_941, 982_451_653, 452_930_477, 472_882_027]

    test_elixir_func = fn ->
      huge_primes
      |> Enum.map(fn num ->
        result = PrimeCheck.is_prime(num)
        IO.puts("#{num} is Prime: #{result}")
      end)
    end

    test_rust_func = fn ->
      huge_primes
      |> Enum.map(fn num ->
        IO.puts("Executing...")
        {:ok, result} = Rustyelixir.is_prime(num)
        IO.puts("#{num} is Prime: #{result}")
        result
      end)
    end

    test_parallel_elixir_func = fn ->
      huge_primes
      |> PMap.run(fn num ->
        result = PrimeCheck.is_prime(num)
        IO.puts("#{num} is Prime: #{result}")
        result
      end)
    end

    test_parallel_rust_func = fn ->
      huge_primes
      |> PMap.run(fn num ->
        # Without this it performs same as non-parallel one
        IO.puts("Executing...")
        {:ok, result} = Rustyelixir.is_prime(num)
        IO.puts("#{num} is Prime: #{result}")
        result
      end)
    end

    # IO.puts("FINDING PRIME NUMBERS IN ELIXIR:")
    # IO.puts("")
    # measure(test_elixir_func)
    # IO.puts("")
    IO.puts("-------------------------------")
    IO.puts("FINDING PRIME NUMBERS USING RUST NIF:")
    IO.puts("")
    measure(test_rust_func)
    IO.puts("")
    # IO.puts("-------------------------------")
    # IO.puts("FINDING PRIME NUMBERS PARALLELY IN ELIXIR:")
    # IO.puts("")
    # measure(test_parallel_elixir_func)
    # IO.puts("")
    # IO.puts("-------------------------------")
    IO.puts("")
    IO.puts("FINDING PRIME NUMBERS PARALLELY USING RUST NIF:")
    IO.puts("")
    measure(test_parallel_rust_func)
    IO.puts("")
    IO.puts("-------------------------------")
    IO.puts("")

    # Benchee.run(
    #   %{
    #     "elixir prime" => test_elixir_func,
    #     "rust prime" => test_rust_func,
    #     "elixir parallel prime" => test_parallel_elixir_func,
    #     "rust parallel prime" => test_parallel_rust_func
    #   }
    # )
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
