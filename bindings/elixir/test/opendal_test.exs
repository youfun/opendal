defmodule OpenDALTest do
  use ExUnit.Case
  doctest OpenDAL

  test "memory service operations" do
    {:ok, op} = OpenDAL.new("memory", %{})

    assert :ok = OpenDAL.write(op, "test.txt", "Hello, OpenDAL!")
    assert {:ok, "Hello, OpenDAL!"} = OpenDAL.read(op, "test.txt")
    assert :ok = OpenDAL.stat(op, "test.txt")
    assert :ok = OpenDAL.delete(op, "test.txt")
    # Verify deletion
    assert {:error, _} = OpenDAL.read(op, "test.txt")
  end
end
