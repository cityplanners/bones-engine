use wgpu::util::DeviceExt;
use cgmath::Vector3;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointLightUniform {
    pub position: [f32; 3],
    pub ambient_intensity: f32,
    pub color: [f32; 3],
    pub diffuse_intensity: f32
}

pub struct PointLight {
    pub uniform: PointLightUniform,
    pub(crate) light_bind_group: Option<wgpu::BindGroup>
}

impl PointLight {
    pub fn new() -> Self {
        PointLight {
            uniform: PointLightUniform {
                position: [0.0, 0.0, 0.0],
                ambient_intensity: 0.1,
                color: [1.0, 1.0, 1.0],
                diffuse_intensity: 1.0,
            },
            light_bind_group: None
        }
    }
    
    /// Set the RGB color value based on 0-255
    // pub fn set_color(red: u8, ) {
        
    // }
    
    pub(crate) fn create_light_bind_group(&mut self, device: wgpu::Device) {
        
        let light_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });
        
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Light VB"),
                contents: bytemuck::cast_slice(&[self.uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        
        self.light_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        }));

    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DirectionalLight {
    pub direction: [f32; 3],
    pub ambient_intensity: f32,
    pub color: [f32; 3],
    pub diffuse_intensity: f32
}