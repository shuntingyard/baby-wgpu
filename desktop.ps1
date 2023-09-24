$env:RUST_LOG = "trace,wgpu_core=info"
cargo run
Remove-Item Env:\RUST_LOG
