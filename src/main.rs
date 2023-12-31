use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

use baby_wgpu::run;

fn main() {
    // Have a quick glance at adapters available (before tracing).
    wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    })
    .enumerate_adapters(wgpu::Backends::all())
    .for_each(|adapter| {
        let attr = adapter.get_info();
        println!(
            "{:>70}, {:?}, {:?}",
            attr.name, attr.backend, attr.device_type
        );
    });

    // Subscribe to traces.
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env()) // Read trace levels from RUST_LOG env var.
        .init();

    // Enable async here by putting the tread into a wait state
    // while all can still be handled differently for wasm.
    pollster::block_on(run());
}
