# brutonavenue

```bash
cargo run --color=always --package brutonavenue --bin brutonavenue
```

```bash
grpcui -plaintext -import-path ./proto -proto brutonavenue.proto 127.0.0.1:5051
```

```bash
grpcurl -import-path ./proto -proto brutonavenue.proto -plaintext -d @ 127.0.01:5051 brutonavenue.BrutonavenueService/SayHello << EOM
{
  "id" : 1,
  "name": "john"
}
EOM
```
