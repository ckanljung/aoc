defmodule AOCTest do
  use ExUnit.Case
  doctest AOC

  test "increase reference counter" do
    assert AOC.increase_ref({:nop, 2, 0}) == {:nop, 2, 1}
  end
  test "Pop and push empty lists" do
    assert AOC.pop_and_pushn([],[],0) == {[],[]}
  end
  test "Pop and push zero elements" do
    assert AOC.pop_and_pushn([1,2],[3,4],0) == {[1,2],[3,4]}
  end
  test "Pop and push entire list" do
    assert AOC.pop_and_pushn([1,2,3],[],3) == {[],[3,2,1]}
  end
  test "Pop and push partial list" do
    assert AOC.pop_and_pushn([1,2,3,4],[],2) == {[3,4],[2,1]}
  end
  test "Pop and push single elem" do
    assert AOC.pop_and_pushn([3,4,5],[2,1],1) == {[4,5],[3,2,1]}
  end

  test "Single nop" do
    assert AOC.exec_vm([], [{:nop, 3, 0}], 7) == 7
  end
  test "Single acc" do
    assert AOC.exec_vm([], [{:acc, 3, 0}], 7) == 10
  end
  test "mulit acc" do
    assert AOC.exec_vm([], [{:acc, 1, 0}, {:acc, 2, 0}], 0) == 3
  end
  test "break on refcount" do
    assert AOC.exec_vm([], [{:acc, 1, 0}, {:acc, 2, 0}, {:acc, 4, 1}], 0) == 3
  end
  test "jmp +1" do
    assert AOC.exec_vm([], [{:acc, 1, 0}, {:jmp, 1, 0}, {:acc, 4, 0}], 0) == 5
  end
  test "jmp +2" do
    assert AOC.exec_vm([], [{:acc, 1, 0}, {:jmp, 2, 0}, {:acc, 4, 0}, {:acc, 8, 0}], 0) == 9
  end
  test "jmp -1" do
    assert AOC.exec_vm([], [{:acc, 1, 0}, {:jmp, -1, 0}, {:acc, 4, 0}], 0) == 1
  end
  test "jmp -2" do
    assert AOC.exec_vm([], [{:acc, 1, 0}, {:acc, 2, 0}, {:jmp, -2, 0}, {:acc, 4, 0}], 0) == 3
  end
  test "jmp forwards and backwards" do
    assert AOC.exec_vm([], [{:acc, 1, 0}, {:jmp, 2, 0}, {:acc, 2, 0}, {:acc, 4, 0}, {:jmp, -2, 0}, {:acc, 8, 0}], 0) == 5
  end
  test "main test case" do
    assert AOC.run08("test/data/08/input.txt") == 5
  end



end
