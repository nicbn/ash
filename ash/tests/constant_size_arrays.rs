use ash;

use ash::vk::{PhysicalDeviceProperties, PipelineColorBlendStateCreateInfo};

#[test]
fn assert_struct_field_is_array() {
    let pipeline_cache_uuid: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let _ = PhysicalDeviceProperties::builder().pipeline_cache_uuid(pipeline_cache_uuid);

    let _ = PhysicalDeviceProperties {
        pipeline_cache_uuid,
        ..Default::default()
    };

    let blend_constants: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

    let _ = PipelineColorBlendStateCreateInfo::builder().blend_constants(blend_constants);

    let _ = PipelineColorBlendStateCreateInfo {
        blend_constants,
        ..Default::default()
    };
}

#[test]
#[allow(dead_code)]
fn assert_ffi_array_param_is_pointer() {
    use ash::version::DeviceV1_0;
    unsafe {
        // don't run it, just make sure it compiles
        if false {
            let device: ash::Device = std::mem::zeroed();
            let cmd_buffer = std::mem::zeroed();

            let blend_constants: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

            device.cmd_set_blend_constants(cmd_buffer, &blend_constants);
        }
    }
}
