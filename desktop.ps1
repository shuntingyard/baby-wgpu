$env:RUST_LOG = "trace,wgpu_core=info,wgpu_core::device::global=warn,wgpu_hal=warn"
cargo run
Remove-Item Env:\RUST_LOG
