defmodule AOC do
  @moduledoc """
  Documentation for `AOC`.
  """

  @doc """

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

  def increase_ref({op,value,ref}) do
    {op,value,ref+1}
  end

  def exec_vm(_, [], accumulator), do: accumulator
  def exec_vm(processed, [rest_head | rest_tail], accumulator) do 
    case rest_head do
      {_, _, 1} -> accumulator
      { :nop, _, _} -> exec_vm( [increase_ref(rest_head)|processed], rest_tail, accumulator )
      { :acc, value, _} -> exec_vm( [increase_ref(rest_head)|processed], rest_tail, accumulator+value )
      { :jmp, value, _} when value >= 0 -> 
        {rest, processed} = pop_and_pushn([increase_ref(rest_head)|rest_tail], processed, value)
        exec_vm( [increase_ref(rest_head)|processed], rest, accumulator)
      { :jmp, value, _} when value < 0 -> 
        {rest, processed} = pop_and_pushn(processed, [increase_ref(rest_head)|rest_tail], value)
        exec_vm( [increase_ref(rest_head)|processed], rest, accumulator)
    end
  end

  _def_file = "lib/data/08/input.txt"


  def run08(file) do
    istr_list = read_input(file) |> Enum.map(&parse_istr/1)
    exec_vm([], istr_list, 0)
  end

  def read_input(file) do
    _fixed_contents =
      File.stream!(file)
      |> Enum.map(&String.trim/1)
  end
end
