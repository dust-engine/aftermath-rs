# Aftermath-rs
Rust bindings for the NVIDIA Aftermath SDK, targeting Vulkan applications.

This crate helps you to obtain NVIDIA Aftermath GPU dump files after a device lost event.

Supports both Windows and Linux. Closed-source binaries are automatically downloaded and statically linked.


## Usage
```rs
extern crate aftermath_rs as aftermath;
use ash::vk;

struct Delegate;
impl aftermath::AftermathDelegate for Delegate {
    fn dumped(&mut self, dump_data: &[u8]) {
        // Write `dump_data` to file, or send to telemetry server
    }
    fn shader_debug_info(&mut self, data: &[u8]) {
    }

    fn description(&mut self, describe: &mut aftermath::DescriptionBuilder) {
    }
}

let _guard = aftermath::Aftermath::new(Delegate);

fn handle_error(error: vk::Result) -> vk::Result {
    let status = aftermath::Status::wait_for_status(Some(std::time::Duration::from_secs(5)));
    if status != aftermath::Status::Finished {
        panic!("Unexpected crash dump status: {:?}", status);
    }
    std::process::exit(1);
}

// Make Vulkan API Calls
device.queue_submit(..)
    .map_err(handle_error)
    .unwrap();

```
