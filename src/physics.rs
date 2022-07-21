use macroquad::prelude::{draw_line, vec2, RED};

use crate::graph::Graph;

impl Graph {
    pub fn update_positions(&mut self, frametime: f32) {
        const EQUILIBRIUM_LEN: f32 = 4.;
        const STIFFNESS: f32 = 1.;
        const AIR_RESISTANCE: f32 = 0.95;

        const ORIGIN_FORCE: f32 = 0.5;

        for node_id in 0..self.nodes.len() {
            let mut vel_acc = vec2(0., 0.);
            let node = &self.nodes[node_id];

            // Repell from every other node.
            for other_node_id in 0..self.nodes.len() {
                if other_node_id == node_id {
                    continue;
                }

                let other_node = &self.nodes[other_node_id];

                let dir = -(other_node.pos - node.pos).normalize();
                let distance = other_node.pos.distance(node.pos);
                let force = 20. * f32::exp(-0.00001 * distance);
                vel_acc += dir * force;
            }
            vel_acc = vel_acc.normalize() * 20000.;

            // Spring force for every neighbor
            for &neighbor_id in &node.neighbors {
                let neighbor = &self.nodes[neighbor_id];

                let distance = node.pos.distance(neighbor.pos);
                let delta = distance - EQUILIBRIUM_LEN;
                let force = STIFFNESS * delta;

                let dir = (neighbor.pos - node.pos).normalize();
                vel_acc += dir * force;
            }

            // Push towards center.
            vel_acc += -node.pos.normalize() * ORIGIN_FORCE;

            let node = &mut self.nodes[node_id];
            node.vel += vel_acc * frametime;
            node.vel *= AIR_RESISTANCE;

            node.pos += node.vel * frametime;
            draw_line(
                node.pos.x,
                node.pos.y,
                (node.pos + node.vel).x,
                (node.pos + node.vel).y,
                1.,
                RED,
            );
        }
    }
}
