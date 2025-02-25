use na::Point3;
use rand::distributions::{Distribution, Standard};
use rand::{rngs::StdRng, SeedableRng};
use rapier3d::dynamics::{JointSet, RigidBodyBuilder, RigidBodySet};
use rapier3d::geometry::{ColliderBuilder, ColliderSet};
use rapier_testbed3d::Testbed;

pub fn init_world(testbed: &mut Testbed) {
    /*
     * World
     */
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let joints = JointSet::new();

    /*
     * Ground
     */
    let ground_size = 40.0;
    let ground_height = 0.1;

    let rigid_body = RigidBodyBuilder::new_static()
        .translation(0.0, -ground_height, 0.0)
        .build();
    let handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(ground_size, ground_height, ground_size).build();
    colliders.insert(collider, handle, &mut bodies);

    /*
     * Create the polyhedra
     */
    let num = 5;
    let scale = 2.0;
    let border_rad = 0.1;

    let shift = border_rad * 2.0 + scale;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;
    let centerz = shift * (num / 2) as f32;

    let mut rng = StdRng::seed_from_u64(0);
    let distribution = Standard;

    for j in 0usize..25 {
        for i in 0..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx;
                let y = j as f32 * shift + centery + 3.0;
                let z = k as f32 * shift - centerz;

                let mut points = Vec::new();
                for _ in 0..10 {
                    let pt: Point3<f32> = distribution.sample(&mut rng);
                    points.push(pt * scale);
                }

                // Build the rigid body.
                let rigid_body = RigidBodyBuilder::new_dynamic().translation(x, y, z).build();
                let handle = bodies.insert(rigid_body);
                let collider = ColliderBuilder::round_convex_hull(&points, border_rad)
                    .unwrap()
                    .build();
                colliders.insert(collider, handle, &mut bodies);
            }
        }
    }

    /*
     * Set up the testbed.
     */
    testbed.set_world(bodies, colliders, joints);
    testbed.look_at(Point3::new(30.0, 30.0, 30.0), Point3::origin());
}
