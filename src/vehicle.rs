#![allow(dead_code)]

use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::control::VehicleControl;
use crate::track::StartPose;

#[derive(Component)]
pub struct Vehicle;

pub fn spawn_vehicle(
    commands: &mut Commands,
    transform: Transform,
    color: Color,
    size: Vec2,
    label: impl Into<String>,
) {
    let half_extents = size * 0.5;

    commands.spawn((
        Vehicle,
        VehicleControl::default(),
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..Default::default()
            },
            transform,
            ..Default::default()
        },
        RigidBody::Dynamic,
        Velocity::default(),
        Damping {
            linear_damping: 4.0,
            angular_damping: 2.5,
        },
        Collider::cuboid(half_extents.x, half_extents.y),
        Friction {
            coefficient: 1.5,
            combine_rule: CoefficientCombineRule::Min,
        },
        Restitution::coefficient(0.05),
        Name::new(label.into()),
    ));
}

pub fn grid_start_positions(
    start: StartPose,
    total: usize,
    per_row: usize,
    forward_spacing: f32,
    lateral_spacing: f32,
) -> Vec<(Vec2, f32)> {
    if total == 0 {
        return Vec::new();
    }

    let per_row = per_row.max(1);
    let forward = Vec2::new(start.direction.cos(), start.direction.sin());
    let right = Vec2::new(forward.y, -forward.x);
    let mut positions = Vec::with_capacity(total);

    for index in 0..total {
        let row = index / per_row;
        let column = index % per_row;
        let lateral_offset = column as f32 - (per_row as f32 - 1.0) * 0.5;
        let position = start.position - forward * (row as f32 * forward_spacing)
            + right * (lateral_offset * lateral_spacing);
        positions.push((position, start.direction));
    }

    positions
}
