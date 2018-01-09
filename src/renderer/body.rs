use glium::{VertexBuffer, Surface, Program, Display, DrawParameters, Depth, DepthTest, Blend, BlendingFunction, LinearBlendingFactor, index};
use super::super::simulation::Body;
use super::scene::Scene;
use std::cmp::Ordering;
use cgmath::Vector4;

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
        let params = scene.get_params();
        let view_look_at = scene.get_view() * params.look_at.extend(1.0);
        let uniforms = uniform! {
            projection: Into::<[[f32; 4]; 4]>::into(scene.get_projection()),
            view: Into::<[[f32; 4]; 4]>::into(scene.get_view()),
            res: [scene.get_width(), scene.get_height()],
            look_at: Into::<[f32; 4]>::into(view_look_at),
            focus: params.focus,
            far: params.far,
            near: params.near
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
            let v1 = scene.get_view() * Vector4::new(v1[0], v1[1], v1[2], 1.0);
            let v2 = scene.get_view() * Vector4::new(v2[0], v2[1], v2[2], 1.0);
            v1.z.partial_cmp(&v2.z).unwrap_or(Ordering::Less)
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