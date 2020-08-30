defmodule PrimeCheck do
  def is_Prime(2), do: true

  def is_prime(num) do
    !Enum.any?(2..(num - 1), fn x -> rem(num, x) == 0 end)
  end
end
