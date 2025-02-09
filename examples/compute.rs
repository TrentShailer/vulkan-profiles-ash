use core::slice;
use std::{borrow::Cow, ffi::CStr, io::Cursor, time::Instant};

use ash::{
    ext,
    util::{read_spv, Align},
    vk,
};
use rand::Rng;
use rand_distr::Distribution;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use vp_ash::vp;

const TRY_DEBUG: bool = true;
// Maximum memory = (2^26 * 2) + (((2^26 / (64 * 4)) * (64 / 4)) * 2) = 142606336
const BUFFER_VALUES: u32 = 2u32.pow(26);
const COMMAND_BUFFER_COUNT: u32 = 3;

fn n_dispatches(n_values_input: u32, subgroup_size: u32) -> u32 {
    let values_processed_per_dispatch = 128 * subgroup_size;

    n_values_input.div_ceil(values_processed_per_dispatch)
}

fn n_values_output(n_values_input: u32, subgroup_size: u32) -> u32 {
    let subgroups_per_dispatch = 128 / subgroup_size; // 16
    let values_processed_per_dispatch = 128 * subgroup_size; // 1024
    let values_produced_per_dispatch = subgroups_per_dispatch; // 16

    let number_of_dispatches = n_values_input as f64 / values_processed_per_dispatch as f64;

    (number_of_dispatches * values_produced_per_dispatch as f64).ceil() as u32
}

#[repr(C)]
struct PushConstants {
    pub input_length: u32,
}

fn main() {
    // Setup objects.
    let entry = ash::Entry::linked();
    let vp_entry = vp_ash::Entry::linked();
    let capabilities = {
        let create_info = vp::CapabilitiesCreateInfo::default()
            .api_version(vk::make_api_version(0, 1, 2, 198))
            .flags(vp::CapabilitiesCreateFlags::STATIC);

        unsafe { vp_entry.create_capabilities(&create_info, None) }.unwrap()
    };

    // Profiles for this application.
    let core_profile = vp::ProfileProperties::default()
        .profile_name(c"VP_VPA_examples_compute")
        .unwrap()
        .spec_version(1);
    let debug_profile = vp::ProfileProperties::default()
        .profile_name(c"VP_VPA_examples_debug")
        .unwrap()
        .spec_version(1);

    // Sanity check that profiles are present, if the instance in the build environment is missing
    // the required extensions, this will fail.
    {
        let profiles = unsafe { capabilities.get_profiles() }.unwrap();

        assert!(
            profiles.contains(&core_profile),
            "The build environment does not support the profiles."
        );
        assert!(
            profiles.contains(&debug_profile),
            "The build environment does not support the profiles."
        );
    };

    // Check for instance support.
    let supports_instance =
        unsafe { capabilities.get_instance_profile_support(None, &core_profile) }.unwrap();
    if !supports_instance {
        panic!("Your Vulkan Instance does not meet the requirements to run this application. Try updating your drivers.")
    }

    // If the instance supports debug and debug is wanted, then we should debug.
    let should_debug = {
        let supports_debug =
            unsafe { capabilities.get_instance_profile_support(None, &debug_profile) }.unwrap();

        TRY_DEBUG && supports_debug
    };

    // Create the list of profiles to use.
    let mut enabled_profiles = vec![core_profile];
    if should_debug {
        enabled_profiles.push(debug_profile);
    }

    // Create instance.
    let instance = {
        let api_version = unsafe { capabilities.get_profile_api_version(&core_profile) };

        let app_info = vk::ApplicationInfo::default()
            .api_version(api_version)
            .application_name(c"VPA Compute Example");

        let vk_create_info = vk::InstanceCreateInfo::default().application_info(&app_info);

        let vp_create_info = vp::InstanceCreateInfo::default()
            .create_info(&vk_create_info)
            .enabled_full_profiles(&enabled_profiles);

        unsafe { capabilities.create_instance(&entry, &vp_create_info, None) }.unwrap()
    };

    // Select a physical device.
    let physical_device = {
        unsafe { instance.enumerate_physical_devices() }
            .unwrap()
            .into_iter()
            .filter(|&device| unsafe {
                capabilities
                    .get_physical_device_profile_support(&instance, device, &core_profile)
                    .unwrap()}
            )
            .min_by_key(|&device| {
                let properties = unsafe {instance.get_physical_device_properties(device)};

                match properties.device_type {
                    vk::PhysicalDeviceType::DISCRETE_GPU => 0,
                    vk::PhysicalDeviceType::INTEGRATED_GPU => 1,
                    vk::PhysicalDeviceType::VIRTUAL_GPU => 2,
                    vk::PhysicalDeviceType::CPU => 3,
                    vk::PhysicalDeviceType::OTHER => 4,
                    _ => 5,
                }
            }).expect("No GPU in your system meets the requirements to run this application. Try updating your drivers.")
    };

    // Get the queue family index.
    let queue_family_index = {
        // Get the required properties
        let required_properties = {
            let mut required_properties = vk::QueueFamilyProperties2KHR::default();
            let mut count = 1;
            unsafe {
                capabilities
                    .get_profile_queue_family_properties(
                        &core_profile,
                        None,
                        &mut count,
                        Some(core::slice::from_mut(&mut required_properties)),
                    )
                    .unwrap()
            };

            required_properties.queue_family_properties
        };

        // Get the device properties
        let properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        // Find the index that meets the requirements.
        properties
            .into_iter()
            .position(|properties| {
                properties.queue_count >= required_properties.queue_count
                    && properties
                        .queue_flags
                        .contains(required_properties.queue_flags)
            })
            .unwrap() as u32
    };

    // Create logical device.
    let device = {
        let queue_create_infos = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&[1.0; 1])];

        let vk_create_info =
            vk::DeviceCreateInfo::default().queue_create_infos(&queue_create_infos);

        let vp_create_info = vp::DeviceCreateInfo::default()
            .create_info(&vk_create_info)
            .enabled_full_profiles(&enabled_profiles);

        unsafe { capabilities.create_device(&instance, physical_device, &vp_create_info, None) }
            .unwrap()
    };

    // Retrieve the queue.
    let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

    // Create debug utils if we should debug
    let debug_utils = if should_debug {
        let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                    | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(vulkan_debug_callback));

        let dbg_instance = ext::debug_utils::Instance::new(&entry, &instance);
        let messenger =
            unsafe { dbg_instance.create_debug_utils_messenger(&debug_info, None) }.unwrap();

        let device = ext::debug_utils::Device::new(&instance, &device);

        Some((dbg_instance, messenger, device))
    } else {
        None
    };

    println!("Created vulkan context");

    try_name(debug_utils.as_ref(), queue, "Main Queue");

    // Create transient command pool
    let transient_command_pool = {
        let create_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(queue_family_index)
            .flags(vk::CommandPoolCreateFlags::TRANSIENT);
        let command_pool = unsafe { device.create_command_pool(&create_info, None) }.unwrap();

        try_name(debug_utils.as_ref(), command_pool, "Transient Command Pool");

        command_pool
    };

    // Create executor command pools and buffers
    let command_objects: Vec<_> = {
        (0..COMMAND_BUFFER_COUNT)
            .map(|index| {
                let pool_create_info =
                    vk::CommandPoolCreateInfo::default().queue_family_index(queue_family_index);
                let command_pool =
                    unsafe { device.create_command_pool(&pool_create_info, None) }.unwrap();

                let command_buffer_info = vk::CommandBufferAllocateInfo::default()
                    .command_pool(command_pool)
                    .level(vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(1);
                let command_buffer =
                    unsafe { device.allocate_command_buffers(&command_buffer_info) }.unwrap()[0];

                // Debug: Name the objects.
                try_name(
                    debug_utils.as_ref(),
                    command_pool,
                    &format!("Exec Pool {index}"),
                );
                try_name(
                    debug_utils.as_ref(),
                    command_buffer,
                    &format!("Exec Buffer {index}"),
                );

                (command_pool, command_buffer)
            })
            .collect()
    };

    // Create timeline semaphore
    let semaphore = {
        let mut type_info = vk::SemaphoreTypeCreateInfo::default()
            .initial_value(0)
            .semaphore_type(vk::SemaphoreType::TIMELINE);
        let create_info = vk::SemaphoreCreateInfo::default().push_next(&mut type_info);

        unsafe { device.create_semaphore(&create_info, None) }.unwrap()
    };

    // Create descriptor pool
    let descriptor_pool = {
        let pool_size = vk::DescriptorPoolSize::default()
            .descriptor_count(2)
            .ty(vk::DescriptorType::STORAGE_BUFFER);
        let create_info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(2)
            .pool_sizes(slice::from_ref(&pool_size));

        unsafe { device.create_descriptor_pool(&create_info, None) }.unwrap()
    };

    // Create descriptor sets
    let (descriptor_set_layout, descriptor_sets) = {
        let set_layout = {
            let bindings = [
                vk::DescriptorSetLayoutBinding::default()
                    .binding(0)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::COMPUTE),
                vk::DescriptorSetLayoutBinding::default()
                    .binding(1)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::COMPUTE),
            ];
            let layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);

            unsafe { device.create_descriptor_set_layout(&layout_info, None) }.unwrap()
        };

        let sets = {
            let layouts = slice::from_ref(&set_layout).repeat(2);
            let allocate_info = vk::DescriptorSetAllocateInfo::default()
                .descriptor_pool(descriptor_pool)
                .set_layouts(&layouts);

            unsafe { device.allocate_descriptor_sets(&allocate_info) }.unwrap()
        };

        try_name(debug_utils.as_ref(), sets[0], "Read-Write Set");
        try_name(debug_utils.as_ref(), sets[1], "Write-Read Set");

        (set_layout, sets)
    };

    // Create pipeline
    let (pipeline_layout, shader_module, pipeline) = {
        let push_range = vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::COMPUTE)
            .offset(0)
            .size(size_of::<PushConstants>() as u32);

        let pipeline_layout = {
            let layout_info = vk::PipelineLayoutCreateInfo::default()
                .push_constant_ranges(slice::from_ref(&push_range))
                .set_layouts(slice::from_ref(&descriptor_set_layout));

            unsafe { device.create_pipeline_layout(&layout_info, None) }.unwrap()
        };

        let shader_module = {
            let bytes = include_bytes!("shaders/maximum_reduction.spv");
            let mut cursor = Cursor::new(bytes);
            let shader_code = read_spv(&mut cursor).unwrap();
            let shader_info = vk::ShaderModuleCreateInfo::default().code(&shader_code);
            unsafe { device.create_shader_module(&shader_info, None) }.unwrap()
        };

        let pipeline = {
            let create_info = vk::ComputePipelineCreateInfo::default()
                .stage(
                    vk::PipelineShaderStageCreateInfo::default()
                        .stage(vk::ShaderStageFlags::COMPUTE)
                        .module(shader_module)
                        .name(c"main"),
                )
                .layout(pipeline_layout);

            unsafe {
                device.create_compute_pipelines(
                    vk::PipelineCache::null(),
                    slice::from_ref(&create_info),
                    None,
                )
            }
            .unwrap()[0]
        };

        (pipeline_layout, shader_module, pipeline)
    };

    // Get the subgroup size
    let subgroup_size = {
        let mut subgroup_properties = vk::PhysicalDeviceSubgroupProperties::default();
        let mut properties =
            vk::PhysicalDeviceProperties2::default().push_next(&mut subgroup_properties);
        unsafe { instance.get_physical_device_properties2(physical_device, &mut properties) };

        subgroup_properties.subgroup_size
    };

    // Setup buffer
    let data_size = BUFFER_VALUES as u64 * size_of::<i32>() as u64;
    let first_output_size =
        n_values_output(BUFFER_VALUES, subgroup_size) as u64 * size_of::<i32>() as u64;
    let (buffer, memory) = {
        let buffer_bytes = data_size + first_output_size;

        unsafe {
            allocate_buffer(
                &instance,
                &device,
                physical_device,
                buffer_bytes,
                vk::BufferUsageFlags::TRANSFER_DST
                    | vk::BufferUsageFlags::TRANSFER_SRC
                    | vk::BufferUsageFlags::STORAGE_BUFFER,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )
        }
    };
    try_name(debug_utils.as_ref(), buffer, "Main Buffer");
    try_name(debug_utils.as_ref(), memory, "Main Buffer Memory");

    println!("Initialised Vulkan");

    // ---- Vulkan Init Completed ----

    // Create data
    let data = {
        let true_max_index = rand::thread_rng().gen_range(0..BUFFER_VALUES as usize);

        let distribution = rand_distr::Uniform::new(i32::MIN, i32::MAX); // Excludes i32::MAX
        let data: Vec<_> = (0..BUFFER_VALUES as usize)
            .into_par_iter()
            .map_init(rand::thread_rng, |rng, index| {
                if index == true_max_index {
                    i32::MAX
                } else {
                    distribution.sample(rng)
                }
            })
            .collect();

        data
    };

    println!("Created data");

    // Copy data to GPU
    {
        // Create staging buffer
        let (staging_buffer, staging_memory) = unsafe {
            allocate_buffer(
                &instance,
                &device,
                physical_device,
                data_size,
                vk::BufferUsageFlags::TRANSFER_SRC,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            )
        };

        try_name(debug_utils.as_ref(), staging_buffer, "Staging Buffer A");
        try_name(
            debug_utils.as_ref(),
            staging_memory,
            "Staging Buffer A Memory",
        );

        // Copy data to staging
        {
            let pointer = unsafe {
                device.map_memory(staging_memory, 0, data_size, vk::MemoryMapFlags::empty())
            }
            .unwrap();

            let mut align: Align<i32> =
                unsafe { Align::new(pointer, align_of::<i32>() as u64, data_size) };
            align.copy_from_slice(&data);

            unsafe { device.unmap_memory(staging_memory) };
        }

        // Copy staging to gpu
        {
            let command_buffer = unsafe { begin_onetime_command(&device, transient_command_pool) };

            let buffer_copy = vk::BufferCopy::default().size(data_size);
            unsafe {
                device.cmd_copy_buffer(
                    command_buffer,
                    staging_buffer,
                    buffer,
                    slice::from_ref(&buffer_copy),
                )
            };

            unsafe {
                end_singletime_command(&device, queue, transient_command_pool, command_buffer)
            };
        }

        unsafe { device.destroy_buffer(staging_buffer, None) };
        unsafe { device.free_memory(staging_memory, None) };
    }

    // Update descriptor sets
    {
        let read_descriptor = vk::DescriptorBufferInfo::default()
            .buffer(buffer)
            .offset(0)
            .range(data_size);
        let write_descriptor = vk::DescriptorBufferInfo::default()
            .buffer(buffer)
            .offset(data_size)
            .range(first_output_size);

        let writes = [
            vk::WriteDescriptorSet::default()
                .dst_set(descriptor_sets[0])
                .dst_binding(0)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(slice::from_ref(&read_descriptor)),
            vk::WriteDescriptorSet::default()
                .dst_set(descriptor_sets[0])
                .dst_binding(1)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(slice::from_ref(&write_descriptor)),
            // inverse
            vk::WriteDescriptorSet::default()
                .dst_set(descriptor_sets[1])
                .dst_binding(0)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(slice::from_ref(&write_descriptor)),
            vk::WriteDescriptorSet::default()
                .dst_set(descriptor_sets[1])
                .dst_binding(1)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(slice::from_ref(&read_descriptor)),
        ];

        unsafe { device.update_descriptor_sets(&writes, &[]) };
    }

    println!("Completed Data Init");

    // ----- Data Init Complete -----

    let start = Instant::now();

    let mut input_length = BUFFER_VALUES;
    let mut dispatches = n_dispatches(input_length, subgroup_size);
    let mut output_length = n_values_output(input_length, subgroup_size);
    let mut data_in_read = true;
    let mut submission_index = 0;
    let mut current_wait_value = 0u64;
    let mut current_signal_value = 1u64;

    while input_length > 1 {
        let descriptor_set = if data_in_read {
            descriptor_sets[0]
        } else {
            descriptor_sets[1]
        };

        // Wait for previous execution
        // {
        //     let wait_info = vk::SemaphoreWaitInfo::default()
        //         .semaphores(slice::from_ref(&semaphore))
        //         .values(slice::from_ref(&current_wait_value));
        //     unsafe { device.wait_semaphores(&wait_info, u64::MAX) }.unwrap();
        // }

        // Reset pool (buffer)
        let (command_pool, command_buffer) = command_objects[submission_index];
        unsafe { device.reset_command_pool(command_pool, vk::CommandPoolResetFlags::empty()) }
            .unwrap();

        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe { device.begin_command_buffer(command_buffer, &begin_info) }.unwrap();

        unsafe {
            // Insert debug label
            if let Some((_, _, device)) = debug_utils.as_ref() {
                let name = format!("Submission {submission_index}\0");
                let label = vk::DebugUtilsLabelEXT::default()
                    .label_name(CStr::from_bytes_until_nul(name.as_bytes()).unwrap());
                device.cmd_begin_debug_utils_label(command_buffer, &label);
            }

            device.cmd_push_constants(
                command_buffer,
                pipeline_layout,
                vk::ShaderStageFlags::COMPUTE,
                0,
                &input_length.to_ne_bytes(),
            );

            device.cmd_bind_descriptor_sets(
                command_buffer,
                vk::PipelineBindPoint::COMPUTE,
                pipeline_layout,
                0,
                slice::from_ref(&descriptor_set),
                &[],
            );

            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, pipeline);

            device.cmd_dispatch(command_buffer, dispatches, 1, 1);

            // End debug label
            if let Some((_, _, device)) = debug_utils.as_ref() {
                device.cmd_end_debug_utils_label(command_buffer);
            }
        }

        unsafe { device.end_command_buffer(command_buffer) }.unwrap();

        let mut semaphore_submit_info = vk::TimelineSemaphoreSubmitInfo::default()
            .wait_semaphore_values(slice::from_ref(&current_wait_value))
            .signal_semaphore_values(slice::from_ref(&current_signal_value));

        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(slice::from_ref(&semaphore))
            .signal_semaphores(slice::from_ref(&semaphore))
            .command_buffers(slice::from_ref(&command_buffer))
            .wait_dst_stage_mask(slice::from_ref(&vk::PipelineStageFlags::COMPUTE_SHADER))
            .push_next(&mut semaphore_submit_info);

        unsafe { device.queue_submit(queue, slice::from_ref(&submit_info), vk::Fence::null()) }
            .unwrap();

        println!(
            "Submission {} | Input {} | Dispatches {} | Output {} | Wait {} | Signal {}",
            submission_index,
            input_length,
            dispatches,
            output_length,
            current_wait_value,
            current_signal_value
        );

        input_length = output_length;
        dispatches = n_dispatches(input_length, subgroup_size);
        output_length = n_values_output(input_length, subgroup_size);
        data_in_read = !data_in_read;
        submission_index = (submission_index + 1) % COMMAND_BUFFER_COUNT as usize;

        current_wait_value = current_signal_value;
        current_signal_value += 1;
    }

    // wait for final submission
    {
        let wait_info = vk::SemaphoreWaitInfo::default()
            .semaphores(slice::from_ref(&semaphore))
            .values(slice::from_ref(&current_wait_value));
        unsafe { device.wait_semaphores(&wait_info, u64::MAX) }.unwrap();
    }

    println!("Submissions completed");

    // Copy result to cpu
    let maximum = {
        let elements = 1;
        let staging_size = size_of::<i32>() as u64 * elements;

        // Create staging buffer
        let (staging_buffer, staging_memory) = unsafe {
            allocate_buffer(
                &instance,
                &device,
                physical_device,
                staging_size,
                vk::BufferUsageFlags::TRANSFER_DST,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            )
        };

        try_name(debug_utils.as_ref(), staging_buffer, "Staging Buffer B");
        try_name(
            debug_utils.as_ref(),
            staging_memory,
            "Staging Buffer B Memory",
        );

        // Copy staging to staging
        {
            let command_buffer = unsafe { begin_onetime_command(&device, transient_command_pool) };

            // find result buffer
            let output_offset_bytes = if data_in_read { 0 } else { data_size };

            let buffer_copy = vk::BufferCopy::default()
                .size(staging_size)
                .src_offset(output_offset_bytes);

            unsafe {
                device.cmd_copy_buffer(
                    command_buffer,
                    buffer,
                    staging_buffer,
                    slice::from_ref(&buffer_copy),
                )
            };

            unsafe {
                end_singletime_command(&device, queue, transient_command_pool, command_buffer)
            };
        }

        // Copy data to cpu
        let maximum = {
            let pointer = unsafe {
                device.map_memory(staging_memory, 0, staging_size, vk::MemoryMapFlags::empty())
            }
            .unwrap();

            let raw_output: &[i32] =
                unsafe { slice::from_raw_parts(pointer.cast(), elements as usize) };

            // dbg!(raw_output);

            let maximum = raw_output[0];

            unsafe { device.unmap_memory(staging_memory) };

            maximum
        };

        unsafe { device.destroy_buffer(staging_buffer, None) };
        unsafe { device.free_memory(staging_memory, None) };

        maximum
    };

    assert_eq!(maximum, i32::MAX);

    println!(
        "GPU found max {} of {} values in {:.3}ms",
        maximum,
        BUFFER_VALUES,
        start.elapsed().as_secs_f32() * 1000.0
    );

    // Clean up
    unsafe {
        device.destroy_buffer(buffer, None);
        device.free_memory(memory, None);

        for (pool, _) in command_objects {
            device.destroy_command_pool(pool, None);
        }

        device.destroy_command_pool(transient_command_pool, None);

        device.destroy_semaphore(semaphore, None);

        device.destroy_descriptor_set_layout(descriptor_set_layout, None);
        device.destroy_descriptor_pool(descriptor_pool, None);

        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_pipeline(pipeline, None);

        device.destroy_shader_module(shader_module, None);

        device.destroy_device(None);
        if let Some((instance, messenger, _)) = debug_utils {
            instance.destroy_debug_utils_messenger(messenger, None);
        }
        instance.destroy_instance(None);
        capabilities.destroy_capabilities(None);
    }
}

unsafe fn begin_onetime_command(device: &ash::Device, pool: vk::CommandPool) -> vk::CommandBuffer {
    let allocate_info = vk::CommandBufferAllocateInfo::default()
        .command_pool(pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);

    let command_buffer = unsafe { device.allocate_command_buffers(&allocate_info) }.unwrap()[0];

    let begin_info =
        vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    unsafe { device.begin_command_buffer(command_buffer, &begin_info) }.unwrap();

    command_buffer
}

unsafe fn end_singletime_command(
    device: &ash::Device,
    queue: vk::Queue,
    pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
) {
    unsafe { device.end_command_buffer(command_buffer) }.unwrap();

    let fence_info = vk::FenceCreateInfo::default();
    let fence = unsafe { device.create_fence(&fence_info, None) }.unwrap();

    let submit_info = vk::SubmitInfo::default().command_buffers(slice::from_ref(&command_buffer));

    unsafe { device.queue_submit(queue, slice::from_ref(&submit_info), fence) }.unwrap();
    unsafe { device.wait_for_fences(slice::from_ref(&fence), true, u64::MAX) }.unwrap();

    // Cleanup
    unsafe { device.destroy_fence(fence, None) };
    unsafe { device.free_command_buffers(pool, slice::from_ref(&command_buffer)) };
}

fn try_name<H: vk::Handle>(
    debug_utils: Option<&(
        ext::debug_utils::Instance,
        vk::DebugUtilsMessengerEXT,
        ext::debug_utils::Device,
    )>,
    handle: H,
    name: &str,
) {
    if let Some((_, _, device)) = debug_utils.as_ref() {
        let name = format!("{name}\0");
        let pool_name_info = vk::DebugUtilsObjectNameInfoEXT::default()
            .object_handle(handle)
            .object_name(CStr::from_bytes_until_nul(name.as_bytes()).unwrap());

        unsafe { device.set_debug_utils_object_name(&pool_name_info) }.unwrap();
    }
}

fn find_memorytype_index(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    memory_requirements: vk::MemoryRequirements,
    memory_flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    let memory_properties =
        unsafe { instance.get_physical_device_memory_properties(physical_device) };

    memory_properties.memory_types[..memory_properties.memory_type_count as _]
        .iter()
        .enumerate()
        .find(|(index, memory_type)| {
            (1 << index) & memory_requirements.memory_type_bits != 0
                && memory_type.property_flags & memory_flags == memory_flags
        })
        .map(|(index, _memory_type)| index as _)
}

unsafe fn allocate_buffer(
    instance: &ash::Instance,
    device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    size: u64,
    usage: vk::BufferUsageFlags,
    memory_flags: vk::MemoryPropertyFlags,
) -> (vk::Buffer, vk::DeviceMemory) {
    // Create buffer
    let buffer = {
        let create_info = vk::BufferCreateInfo::default().size(size).usage(usage);
        device.create_buffer(&create_info, None).unwrap()
    };

    // Allocate Memory
    let (memory, _memory_requirements) = {
        let memory_requirements_info = vk::BufferMemoryRequirementsInfo2::default().buffer(buffer);

        let mut dedicated_requirements = vk::MemoryDedicatedRequirements::default();
        let mut memory_requirements =
            vk::MemoryRequirements2::default().push_next(&mut dedicated_requirements);

        unsafe {
            device.get_buffer_memory_requirements2(
                &memory_requirements_info,
                &mut memory_requirements,
            )
        };

        let memory_requirements = memory_requirements.memory_requirements;
        let should_be_dedicated = dedicated_requirements.prefers_dedicated_allocation == vk::TRUE;

        let memory_index =
            find_memorytype_index(instance, physical_device, memory_requirements, memory_flags)
                .unwrap();

        let allocate_info = vk::MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_index);

        // Handle allocation or dedicated allocation
        let memory = if should_be_dedicated {
            let mut dedicated_allocation =
                vk::MemoryDedicatedAllocateInfo::default().buffer(buffer);
            let allocate_info = allocate_info.push_next(&mut dedicated_allocation);
            device.allocate_memory(&allocate_info, None)
        } else {
            device.allocate_memory(&allocate_info, None)
        }
        .unwrap();

        (memory, memory_requirements)
    };

    // bind buffer and memory
    unsafe { device.bind_buffer_memory(buffer, memory, 0).unwrap() };

    (buffer, memory)
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let _message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    // Debug
    if callback_data.message_id_number == 1985515673 {
        println!("[DEBUG] {}", message);
        return vk::FALSE;
    }

    let message = message.replace(" | ", "\n");

    match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
            println!("[DEBUG] [{message_type:?}] [{message_id_name}]\n{message}\n")
        }

        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            println!("[INFO] [{message_type:?}] [{message_id_name}]\n{message}\n")
        }

        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            println!("[WARN] [{message_type:?}] [{message_id_name}]\n{message}\n")
        }

        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            println!("[ERROR] [{message_type:?}] [{message_id_name}]\n{message}\n")
        }

        _ => {
            println!("[UNKNOWN] [{message_type:?}] [{message_id_name}]\n{message}\n")
        }
    };

    vk::FALSE
}
