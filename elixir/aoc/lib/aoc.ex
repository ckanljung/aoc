defmodule AOC do
  @moduledoc """
  Documentation for `AOC`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> AOC.hello()
      :world

  """

  def pop_and_pushn( src, dst, 0), do: {src,dst}
  def pop_and_pushn( [], dst, _), do: {[], dst}
  def pop_and_pushn( [srchead | srctail], dst, n), do: pop_and_pushn(srctail, [srchead | dst], n-1)

  def parse_op (op) do
    case op do
      "acc" -> :acc
      "jmp" -> :jmp
      "nop" -> :nop
      _   ->   :undef
    end
  end

  def parse_istr (istr) do
    parselist = Regex.run(~r/([a-z]*)[\s\t]*(\+|-)([0-9]*)/, istr)
    case parselist do
      [_, op, "+", value | _ ] -> { parse_op(op), String.to_integer(value), 0 }
      [_, op, "-", value | _ ] -> { parse_op(op), - String.to_integer(value), 0 }
      [_, _, _, _ | _ ] -> { :undef, 0, 0 }
    end
  end

  def exec_vm({processed, [], accumulator}), do: accumulator
  def exec_vm({processed, [rest_head | rest_tail], accumulator}) do 
    case rest_head do
      {_, _, 1} -> accumulator
      { :nop, value, refcnt} -> exec_vm( [{:nop, value, refcnt+1}|processed], rest_tail, accumulator )
      { :acc, value, refcnt} -> exec_vm( [{:nop, value, refcnt+1}|processed], rest_tail, accumulator+value )
      { :jmp, value, refcnt} -> exec_vm( [{:nop, value, refcnt+1}|processed], rest_tail, accumulator+value )
    end
  end

  _def_file = "lib/data/08/input.txt"
  def read_input(file) do
    _fixed_contents =
      File.stream!(file)
      |> Enum.map(&String.trim/1)
  end
end
