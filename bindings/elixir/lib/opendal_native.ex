defmodule OpenDAL.Native do
  use Rustler, otp_app: :opendal, crate: "opendal_elixir"

  def new(_scheme, _map), do: error()
  def write(_op, _path, _content), do: error()
  def read(_op, _path), do: error()
  def delete(_op, _path), do: error()
  def stat(_op, _path), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
