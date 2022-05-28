use std::{fs::File, io::Read, sync::Arc};
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    device::{
        physical::PhysicalDevice, Device, DeviceCreateInfo, DeviceExtensions, Queue,
        QueueCreateInfo,
    },
    format::Format,
    image::{view::ImageView, AttachmentImage, ImageUsage, SampleCount, SwapchainImage},
    instance::{Instance, InstanceCreateInfo},
    pipeline::{
        graphics::{
            input_assembly::InputAssemblyState,
            vertex_input::{BuffersDefinition, Vertex},
            viewport::{Viewport, ViewportState},
        },
        GraphicsPipeline,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    shader::ShaderModule,
    single_pass_renderpass,
    swapchain::{Surface, Swapchain, SwapchainCreateInfo},
};
use vulkano_win::{required_extensions, VkSurfaceBuild};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct GpuInterface {
    pub instance: Arc<Instance>,
    pub surface: Arc<Surface<Window>>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

impl GpuInterface {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let instance = create_instance();
        let surface = create_surface(instance.clone(), event_loop);
        let (device, queue) = create_device_and_queue(instance.clone(), surface.clone());
        Self {
            instance,
            surface,
            device,
            queue,
        }
    }
}

#[derive(Clone)]
pub struct FixtureCreateInfo {}

pub struct Fixture {
    pub render_pass: Arc<RenderPass>,
    pub frame_buffers: Vec<Arc<Framebuffer>>,
    pub swapchain: Arc<Swapchain<Window>>,
    pub swapchain_images: Vec<Arc<SwapchainImage<Window>>>,
}

impl Fixture {
    pub fn new(_fixture_create_info: &FixtureCreateInfo, gpu_interface: &GpuInterface) -> Self {
        let (swapchain, swapchain_images) = create_swapchain_and_images(
            gpu_interface.device.clone(),
            gpu_interface.surface.clone(),
        );
        let render_pass = create_render_pass(gpu_interface.device.clone());
        let frame_buffers = create_frame_buffers(
            gpu_interface.device.clone(),
            &swapchain_images,
            render_pass.clone(),
            swapchain.image_extent(),
        );
        Self {
            swapchain,
            swapchain_images,
            render_pass,
            frame_buffers,
        }
    }
}

fn create_instance() -> Arc<Instance> {
    let required_extensions = required_extensions();
    Instance::new(InstanceCreateInfo {
        enabled_extensions: required_extensions,
        ..InstanceCreateInfo::default()
    })
    .expect("Could not create instance")
}

fn create_surface(instance: Arc<Instance>, event_loop: &EventLoop<()>) -> Arc<Surface<Window>> {
    WindowBuilder::new()
        .with_title("My Vulkan Window")
        .with_inner_size(LogicalSize::new(1280, 720))
        .build_vk_surface(&event_loop, instance)
        .expect("Unable to create window")
}

fn create_device_and_queue(
    instance: Arc<Instance>,
    surface: Arc<Surface<Window>>,
) -> (Arc<Device>, Arc<Queue>) {
    let required_extensions = DeviceExtensions {
        khr_portability_subset: true,
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };
    let physical_device = PhysicalDevice::enumerate(&instance)
        .find(|physical_device| {
            physical_device
                .supported_extensions()
                .intersection(&required_extensions)
                == required_extensions
                && physical_device
                    .surface_capabilities(surface.as_ref(), Default::default())
                    .expect("Could not get surface capabilities")
                    .supported_usage_flags
                    .color_attachment
        })
        .expect("Could not find physical device");
    let queue_family = physical_device
        .queue_families()
        .find(|queue_family| queue_family.supports_graphics())
        .expect("Could not find queue family which supports graphics");
    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: required_extensions,
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            ..Default::default()
        },
    )
    .expect("Could not create logical device");
    let queue = queues.next().expect("Could not get any queue");
    (device, queue)
}

fn create_swapchain_and_images(
    device: Arc<Device>,
    surface: Arc<Surface<Window>>,
) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
    let image_usage = ImageUsage {
        color_attachment: true,
        ..ImageUsage::none()
    };
    let swapchain_create_info = SwapchainCreateInfo {
        image_usage,
        ..SwapchainCreateInfo::default()
    };
    let (swapchain, images) =
        Swapchain::new(device, surface, swapchain_create_info).expect("Could not create swapchain");
    (swapchain, images)
}

fn create_render_pass(device: Arc<Device>) -> Arc<RenderPass> {
    single_pass_renderpass!(device,
                            attachments: {
                                color: {
                                    load: Clear,
                                    store: DontCare,
                                    format: Format::B8G8R8A8_UNORM,
                                    samples: 8,
                                },
                                output: {
                                    load: Clear,
                                    store: Store,
                                    format: Format::B8G8R8A8_UNORM,
                                    samples: 1,
                                }
                            },
                            pass: {
                                color: [color],
                                depth_stencil: {},
                                resolve: [output],
                            }
    )
    .expect("Could not create render pass")
}

pub fn create_graphics_pipeline<V: Vertex>(
    device: Arc<Device>,
    swapchain_extent: [u32; 2],
    render_pass: Arc<RenderPass>,
) -> Arc<GraphicsPipeline> {
    mod vertex_shader {
        vulkano_shaders::shader! {
            ty: "vertex",
            src: "
                #version 450

                layout(location = 0) in vec2 position;
                layout(location = 0) out vec2 f_position;

                void main() {
                    f_position = vec2(position.x, -position.y);
                    gl_Position = vec4(position, 0.0, 1.0);
                }
                    "
        }
    }
    let vertex_shader_module =
        vertex_shader::load(device.clone()).expect("Could not load vertex shader");
    let mut fragment_shader_bytes = Vec::new();
    File::open("data/frag.spv")
        .expect("Could not find fragment shader spv file")
        .read_to_end(&mut fragment_shader_bytes)
        .expect("Could not read fragment shader spv file");
    let fragment_shader_module =
        unsafe { ShaderModule::from_bytes(device.clone(), &fragment_shader_bytes[..]) }
            .expect("Could not load fragment shader module");
    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [swapchain_extent[0] as f32, swapchain_extent[1] as f32],
        depth_range: 0.0..1.0,
    };
    let pipeline_builder = GraphicsPipeline::start()
        .vertex_input_state(BuffersDefinition::new().vertex::<V>())
        .input_assembly_state(InputAssemblyState::default())
        .vertex_shader(
            vertex_shader_module
                .entry_point("main")
                .expect("Could not find entry point for vertex shader module"),
            (),
        )
        .fragment_shader(
            fragment_shader_module
                .entry_point("main")
                .expect("Could not find entry point for fragment shader module"),
            (),
        )
        .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
        .render_pass(
            Subpass::from(render_pass.clone(), 0)
                .expect("Could not create subpass from render pass"),
        );
    pipeline_builder
        .build(device.clone())
        .expect("Could not build graphics pipeline")
}

fn create_frame_buffers(
    device: Arc<Device>,
    swapchain_images: &Vec<Arc<SwapchainImage<Window>>>,
    render_pass: Arc<RenderPass>,
    dimenstions: [u32; 2],
) -> Vec<Arc<Framebuffer>> {
    let color_image = AttachmentImage::multisampled(
        device.clone(),
        dimenstions,
        SampleCount::Sample8,
        Format::B8G8R8A8_UNORM,
    )
    .expect("Could not create color image");
    let color_image_view =
        ImageView::new_default(color_image.clone()).expect("Could not create color image view");
    swapchain_images
        .iter()
        .map(|image| {
            let swapchain_image_view =
                ImageView::new_default(image.clone()).expect("Could not create image view");
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![color_image_view.clone(), swapchain_image_view],
                    ..FramebufferCreateInfo::default()
                },
            )
            .expect("Could not create framebuffer")
        })
        .collect()
}

pub fn create_vertex_buffer<V: Vertex>(
    device: Arc<Device>,
    verticies: Vec<V>,
) -> Arc<CpuAccessibleBuffer<[V]>> {
    CpuAccessibleBuffer::from_iter(device, BufferUsage::vertex_buffer(), false, verticies)
        .expect("Could not create vertex buffer")
}
