use raytracelib::canvas::Canvas;
use raytracelib::color::Color;
use raytracelib::tuple::{Point, Vector};
use std::fs;
use std::fs::OpenOptions;

struct Environment {
    gravity: Vector,
    wind: Vector,
}

#[derive(Clone)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

fn main() {
    let (width, height) = (900, 550);
    let mut canvas = Canvas::new(width, height);
    let color_red = Color::new(1, 0, 0);
    let speed_factor = 10f64;

    let environment = Environment {
        gravity: Vector::new(0, -0.1, 0),
        wind: Vector::new(-0.01, 0, 0),
    };

    let projectile = Projectile {
        position: Point::new(0, 1, 0),
        velocity: Vector::new(1, 1.18, 0).normalize() * speed_factor,
    };

    let projectile_trajectory = environment.fire_projectile(projectile);

    for projectile in projectile_trajectory {
        canvas.write_pixel(
            projectile.position.x() as i32,
            (height as f64 - projectile.position.y()) as i32,
            color_red,
        );
    }

    let mut buf: Vec<u8> = vec![];
    canvas.canvas_to_ppm(&mut buf).unwrap();

    let ppm = String::from_utf8(buf).unwrap();

    let _ = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("ppm/chapter-02.ppm");

    fs::write("ppm/chapter-02.ppm", ppm).unwrap();

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
}
