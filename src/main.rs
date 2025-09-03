use tracing::*;

fn main() {
	yeditor::init_debug_tools("yeditor=debug").unwrap();
	debug!("Logging started");

	info!("Hello, world!");

	let descriptor = wgpu::InstanceDescriptor::default();
	let instance = wgpu::Instance::new(&descriptor);

	let target = todo!();
	let surface = instance
		.create_surface(target)
		.wrap_err("Couldn't create surface")?;
}
