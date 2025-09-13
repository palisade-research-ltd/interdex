## Apple x86

From aple sillicon apple-darwin to x86_64 apple-darwin

```shell
cargo build --bin datacollector --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/datacollector dist/x86_64-apple-darwin
COPYFILE_DISABLE=1 tar -czpf dist/x86_64-apple-darwin/datacollector.tar.gz -C \
dist/x86_64-apple-darwin datacollector
rm dist/x86_64-apple-darwin
```

from apple-darwin (macOS in Apple sillicon) to x86_64 
