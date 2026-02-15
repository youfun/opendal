# Apache OpenDAL™ Elixir Binding

This is the Elixir binding for [Apache OpenDAL™](https://github.com/apache/opendal). It allows you to access data from various storage services (S3, GCS, AzBlob, Redis, fs, etc.) using a unified data access layer.

## Installation

The binding is currently in development. You can install it by adding `opendal` to your list of dependencies in `mix.exs`:

### From Git (Current Branch)

To use the latest development version from the `elixir-bindings` branch:

```elixir
def deps do
  [
    {:opendal, git: "https://github.com/apache/opendal.git", branch: "elixir-bindings", sparse: "bindings/elixir"}
  ]
end
```

### From Local Path (For Development)

If you have cloned the `opendal` repository locally:

```elixir
def deps do
  [
    {:opendal, path: "/path/to/opendal/bindings/elixir"}
  ]
end
```

## Usage Example

Here is a simple example demonstrating how to create an Operator and perform basic read/write operations.

```elixir
# 1. Initialize Operator
# Use the "memory" backend for testing. In production, you can configure "s3", "fs", etc.
# scheme: "memory", "fs", "s3", ...
# map: configuration parameters for the backend
{:ok, op} = OpenDAL.new("memory", %{})

# 2. Write Data
# Write the string "Hello, OpenDAL!" to the file "test.txt"
:ok = OpenDAL.write(op, "test.txt", "Hello, OpenDAL!")

# 3. Read Data
{:ok, data} = OpenDAL.read(op, "test.txt")
IO.puts("Read data: #{data}")
# Output: Read data: Hello, OpenDAL!

# 4. Get File Metadata (Stat)
# Check if the file exists and get its attributes
case OpenDAL.stat(op, "test.txt") do
  :ok -> IO.puts("File exists")
  {:error, _} -> IO.puts("File does not exist")
end

# 5. Delete File
:ok = OpenDAL.delete(op, "test.txt")

# Verify deletion by reading again
case OpenDAL.read(op, "test.txt") do
  {:error, _} -> IO.puts("File deleted successfully")
  _ -> IO.puts("Delete failed")
end
```

## Supported Backends

OpenDAL supports dozens of services, including but not limited to:

- **Standard**: `ftp`, `http`, `sftp`, `webdav`
- **Object Storage**: `azblob`, `cos`, `gcs`, `obs`, `oss`, `s3`
- **File Storage**: `fs`, `hdfs`, `ipfs`
- **Key-Value**: `redis`, `rocksdb`

For a detailed list, please refer to the official OpenDAL documentation.

## Development and Testing

This project requires a Rust environment.

1.  Clone the repository and enter the directory:
    ```shell
    git clone https://github.com/apache/opendal.git
    cd opendal/bindings/elixir
    ```

2.  Get dependencies:
    ```shell
    mix deps.get
    ```

3.  Run tests:
    ```shell
    mix test
    ```
    `rustler` will automatically compile the underlying Rust code.

## Contributing

Pull Requests and Issues are welcome!
