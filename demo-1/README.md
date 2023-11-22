
# 运行与编译

## 运行
```
# 进入 Cargo.toml 文件所在文件夹运行
cargo run
```

# debug模式编译
```
# 编译
cargp build

# 运行
./target/debug/dmeo-1

```


# 发布模式编译 高性能
```
# 编译
cargo build --release

# 运行
./target/release/dmeo-1

```


# 编译检查 节省时间验证代码
```
➜  demo-1 git:(main) ✗ cargo check          
    Checking demo-1 v0.1.0 (/Users/mac/Documents/rust_project/github.rust_learn/demo-1)
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s

```


# 配置文件 说明
```
1 Cargo.toml 是 cargo 特有的项目数据描述文件。
它存储了项目的所有元配置信息，如果 Rust 开发者希望 Rust 项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建 Cargo.toml。

2 Cargo.lock 文件是 cargo 工具根据同一项目的 toml 文件生成的项目依赖详细清单，因此我们一般不用修改它，只需要对着 Cargo.toml 文件撸就行了。

如果是包文件 Cargo.lock 文件需要 .gitigore 忽略

```


