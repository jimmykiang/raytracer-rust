use crate::tuple::{Point, Vector};

pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

#[derive(Clone)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

fn tick(env: &Environment, p: &Projectile) -> Projectile {
    let position = p.position + (p.velocity);
    let velocity = p.velocity + env.gravity + env.wind;

    return Projectile { position, velocity };
}

impl Environment {
    pub fn fire_projectile(&self, projectile: Projectile) -> Vec<Projectile> {
        let mut projectile_trajectory = Vec::<Projectile>::new();
        let mut current_projectile = Projectile {
            position: projectile.position,
            velocity: projectile.velocity,
        };

        while current_projectile.position.y() >= 0.0 {
            projectile_trajectory.push(current_projectile.clone());
            current_projectile = tick(self, &current_projectile);
        }

        projectile_trajectory
    }
}
