use std::sync::Arc;
use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::{Device, DeviceCreateInfo, Features, QueueCreateInfo};
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::memory::allocator::{GenericMemoryAllocator, GenericMemoryAllocatorCreateInfo,
FreeListAllocator};

fn main() {
    let  library = VulkanLibrary::new().expect("no vulkan found");
    let instance = Instance::new(library, InstanceCreateInfo::default()).expect("failed to make instance");
    let physical = instance
        .enumerate_physical_devices()
        .expect("could not emurate devices")
        .next()
        .expect("no devices available");
    print!("Name is : {}", physical.properties().device_name);
    let queue_family_index = physical
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_, q)| q.queue_flags.graphics)
        .expect("no graphical queue family") as u32;

    let (device, mut queues) = Device::new(
        physical,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
                }],
                ..Default::default()
            },
    )
    .expect("failed to create device");

    let queue = queues.next().unwrap();
    let data: i32 = 12;
    let generic_allocator = <GenericMemoryAllocator<Arc<FreeListAllocator>>>::new_default(device.clone());
    let buffer = CpuAccessibleBuffer::from_data(
        &generic_allocator,
        BufferUsage {
            uniform_buffer: true,
            ..Default::default()
        },
        false,
        data,
    )
    .expect("failed to create buffer");


}


