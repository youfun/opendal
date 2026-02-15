defmodule OpenDAL do
  alias OpenDAL.Native

  def new(scheme, map \\ %{}) do
    Native.new(scheme, map)
  end

  def write(op, path, content) do
    Native.write(op, path, content)
  end

  def read(op, path) do
    Native.read(op, path)
  end

  def delete(op, path) do
    Native.delete(op, path)
  end

  def stat(op, path) do
    Native.stat(op, path)
  end
end
