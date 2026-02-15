# Apache OpenDAL™ Elixir 绑定

这是 [Apache OpenDAL™](https://github.com/apache/opendal) 的 Elixir 绑定。它允许你在 Elixir 项目中使用 OpenDAL 统一的数据访问层来操作各种存储服务（如 S3, GCS, AzBlob, Redis, fs 等）。

## 核心功能

- **统一接口**: 使用相同的 API 操作不同的存储后端。
- **高性能**: 底层使用 Rust 实现，通过 NIF (Native Implemented Functions) 与 Elixir 交互。
- **安全**: 使用 Dirty NIFs 处理 I/O 操作，避免阻塞 Erlang VM 调度器。

## 安装

目前该绑定处于开发阶段。你可以通过 Git 或本地路径在 `mix.exs` 中添加依赖：

```elixir
def deps do
  [
    {:opendal, github: "apache/opendal", sparse: "bindings/elixir"}
  ]
end
```

## 使用示例

以下是一个简单的使用示例，演示了如何创建 Operator 并进行基本的读写操作。

```elixir
# 1. 初始化 Operator
# 这里以 memory 后端为例，实际使用时可以配置 s3, fs 等
# scheme: "memory", "fs", "s3", ...
# map: 后端对应的配置参数
{:ok, op} = OpenDAL.new("memory", %{})

# 2. 写入数据
# 将字符串 "Hello, OpenDAL!" 写入到 "test.txt" 文件
:ok = OpenDAL.write(op, "test.txt", "Hello, OpenDAL!")

# 3. 读取数据
{:ok, data} = OpenDAL.read(op, "test.txt")
IO.puts("Read data: #{data}")
# 输出: Read data: Hello, OpenDAL!

# 4. 获取文件元数据 (Stat)
# 检查文件是否存在以及获取其属性
case OpenDAL.stat(op, "test.txt") do
  :ok -> IO.puts("File exists")
  {:error, _} -> IO.puts("File does not exist")
end

# 5. 删除文件
:ok = OpenDAL.delete(op, "test.txt")

# 再次读取验证删除
case OpenDAL.read(op, "test.txt") do
  {:error, _} -> IO.puts("File deleted successfully")
  _ -> IO.puts("Delete failed")
end
```

## 支持的后端

OpenDAL 支持数十种服务，包括但不限于：

- **Standard**: `ftp`, `http`, `sftp`, `webdav`
- **Object Storage**: `azblob`, `cos`, `gcs`, `obs`, `oss`, `s3`
- **File Storage**: `fs`, `hdfs`, `ipfs`
- **Key-Value**: `redis`, `rocksdb`

详细列表请参考 OpenDAL 官方文档。

## 开发与测试

本项目依赖 Rust 环境。

1.  克隆仓库并进入目录：
    ```shell
    git clone https://github.com/apache/opendal.git
    cd opendal/bindings/elixir
    ```

2.  获取依赖：
    ```shell
    mix deps.get
    ```

3.  运行测试：
    ```shell
    mix test
    ```
    `rustler` 会自动编译底层的 Rust 代码。

## 贡献

欢迎提交 Pull Request 或 Issue！
