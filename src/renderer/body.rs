use glium::{VertexBuffer, Surface, Program, Display, DrawParameters, Depth, DepthTest, Blend, BlendingFunction, LinearBlendingFactor, index};
use super::super::simulation::Body;
use super::scene::Scene;
use std::cmp::Ordering;
use cgmath::Vector4;
use cgmath::prelude::*;

#[derive(Copy, Clone)]
struct BodyVertex {
    position: [f32; 3],
    color: [f32; 3],
    radius: f32,
}

implement_vertex!(BodyVertex, position, color, radius);

pub struct BodyRenderer<'a> {
    vertex_buffer: VertexBuffer<BodyVertex>,
    indices: index::NoIndices,
    draw_parameters: DrawParameters<'a>,
    program: Program,
    max_bodies: usize,
}

impl<'a> BodyRenderer<'a> {
    pub fn new(display: &Display, max_bodies: usize) -> Self {
        let draw_parameters = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: Blend {
                color: BlendingFunction::Addition {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: BlendingFunction::Max,
                constant_value: (0.0, 0.0, 0.0, 0.0),
            },
            ..Default::default()
        };

        let indices = index::NoIndices(index::PrimitiveType::Points);

        BodyRenderer {
            max_bodies,
            indices,
            draw_parameters,
            vertex_buffer: VertexBuffer::empty_dynamic(display, max_bodies)
                .expect("Cannot create body vertex buffer"),
            program: Program::from_source(
                display,
                include_str!("../../assets/body_vertex.glsl"),
                include_str!("../../assets/body_fragment.glsl"),
                Some(include_str!("../../assets/body_geometry.glsl")),
            ).expect("Cannot create body program"),
        }
    }

    pub fn draw<S: Surface>(&mut self, surface: &mut S, scene: &Scene, bodies: &[Body]) {
        // Set model view projection and uniforms
        let mvp = scene.projection * scene.view;
        let uniforms = uniform! {
            mvp: Into::<[[f32; 4]; 4]>::into(mvp),
            res: [scene.width, scene.height]
        };

        // Convert bodies to BodyVertices.
        let mut vertices: Vec<BodyVertex> = bodies.iter().map(|&body| BodyVertex {
            position: body.position.into(),
            color: body.color.into(),
            radius: body.radius(),
        }).collect();

        if bodies.len() > self.max_bodies {
            panic!("Bodies should not get larger with each step!");
        }

        // Sort bodies according to mvp for blending support.
        vertices.sort_by(|v1, v2| {
            let v1 = v1.position;
            let v2 = v2.position;
            let mut v1 = mvp * Vector4::new(v1[0], v1[1], v1[2], 1.0);
            v1 /= v1.w;
            let mut v2 = mvp * Vector4::new(v2[0], v2[1], v2[2], 1.0);
            v2 /= v2.w;
            v2.z.partial_cmp(&v1.z).unwrap_or(Ordering::Less)
        });

        // Write new vertices to vertex buffer.
        self.vertex_buffer.as_mut_slice().write(&vertices);

        // Draw bodies.
        surface.draw(&self.vertex_buffer,
                     &self.indices,
                     &self.program,
                     &uniforms,
                     &self.draw_parameters).expect("Failed to draw body");
    }
}