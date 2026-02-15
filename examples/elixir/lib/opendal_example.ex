defmodule OpendalExample do
  def run do
    IO.puts("Running OpenDAL Elixir Example")

    {:ok, op} = OpenDAL.new("memory", %{})
    IO.puts("Operator created")

    :ok = OpenDAL.write(op, "test.txt", "Hello, OpenDAL!")
    IO.puts("Data written to test.txt")

    {:ok, data} = OpenDAL.read(op, "test.txt")
    IO.puts("Data read from test.txt: #{data}")

    :ok = OpenDAL.delete(op, "test.txt")
    IO.puts("Data deleted from test.txt")

    case OpenDAL.read(op, "test.txt") do
        {:error, _} -> IO.puts("Successfully verified deletion")
        _ -> IO.puts("Failed to delete file")
    end
  end
end
