#[macro_export]
macro_rules! pipeline {
    ($device:expr, $render_pipeline_layout:expr, $shader:expr, $config:expr; $($x:expr),*) => {
        $device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&$render_pipeline_layout),
            vertex: VertexState {
                module: &$shader,
                entry_point: Some("vs_main"),
                // specify vertex buffer layout
                buffers: &[$($x),*],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &$shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: $config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None
        })
    };
}

pub(crate) use pipeline;