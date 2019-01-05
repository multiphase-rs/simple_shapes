extern crate vtkio;
extern crate simple_shapes;

use simple_shapes::{circle_2d, Entity};

fn main() {
    let radius = 0.5;
    let center = (0.0, 0.0);
    let spacing = 0.05;
    let (x1, y1) = circle_2d(center, radius, spacing);
    let z1 = vec![0.; x1.len()];

    let center = (1.0, 0.0);
    let (x2, y2) = circle_2d(center, radius, spacing);
    let z2 = vec![0.; x2.len()];

    let center = (-1.0, 0.0);
    let (x3, y3) = circle_2d(center, radius, spacing);
    let z3 = vec![0.; x3.len()];

    let center = (0.0, -1.0);
    let (x4, y4) = circle_2d(center, radius, spacing);
    let z4 = vec![0.; x4.len()];

    let center = (0.0, 1.0);
    let (x5, y5) = circle_2d(center, radius, spacing);
    let z5 = vec![0.; x5.len()];

    let circle_1 = Entity::from_xyz_rad(x1.clone(), y1, z1, vec![spacing/2.; x1.len()]);
    let circle_2 = Entity::from_xyz_rad(x2.clone(), y2, z2, vec![spacing/2.; x2.len()]);
    let circle_3 = Entity::from_xyz_rad(x3.clone(), y3, z3, vec![spacing/2.; x3.len()]);
    let circle_4 = Entity::from_xyz_rad(x4.clone(), y4, z4, vec![spacing/2.; x4.len()]);
    let circle_5 = Entity::from_xyz_rad(x5.clone(), y5, z5, vec![spacing/2.; x5.len()]);

    circle_1.write_vtk("circle_1.vtk");
    circle_2.write_vtk("circle_2.vtk");
    circle_3.write_vtk("circle_3.vtk");
    circle_4.write_vtk("circle_4.vtk");
    circle_5.write_vtk("circle_5.vtk");
}
