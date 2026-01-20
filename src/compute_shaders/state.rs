use wgpu::Device;

pub struct ComputeState {
    compute_pipeline: wgpu::ComputePipeline,
}
impl ComputeState {
    pub fn new(device: &Device,) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("../dual_contouring/mesh_maker.wgsl"));

        Self {
            compute_pipeline: device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("compute shader"),
                layout: None,
                module: &shader,
                entry_point: None,
                compilation_options: Default::default(),
                cache: Default::default(),
            }),
        }
    }
}