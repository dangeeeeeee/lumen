use ash::{vk, Device};

pub struct GpuTracer {
    pub device: Device,
    pub pipeline: vk::Pipeline,
    pub layout: vk::PipelineLayout,
    pub desc: vk::DescriptorSet,
    pub tris_buf: vk::Buffer,
    pub nodes_buf: vk::Buffer,
    pub out_buf: vk::Buffer,
}

impl GpuTracer {
    pub fn dispatch(&self, cmd: vk::CommandBuffer, w: u32, h: u32) {
        unsafe {
            self.device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::COMPUTE, self.pipeline);
            self.device.cmd_bind_descriptor_sets(
                cmd,
                vk::PipelineBindPoint::COMPUTE,
                self.layout,
                0,
                &[self.desc],
                &[],
            );
            self.device.cmd_dispatch(cmd, (w + 7) / 8, (h + 7) / 8, 1);
            let barrier = vk::MemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::SHADER_WRITE)
                .dst_access_mask(vk::AccessFlags::HOST_READ | vk::AccessFlags::TRANSFER_READ);
            self.device.cmd_pipeline_barrier(
                cmd,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::HOST | vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[barrier],
                &[],
                &[],
            );
        }
    }
}
