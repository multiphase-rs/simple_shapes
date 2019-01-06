// [[file:~/phd/code_phd/simple_shapes/README.org::code:example_zhang_geometry][code:example_zhang_geometry]]
extern crate simple_shapes;
extern crate vtkio;

use simple_shapes::{benchmarks::create_zhang_geometry, Entity};

fn main() {
    let spacing = 0.0005;

    let (xc, yc, _, xt, yt) = create_zhang_geometry(spacing);

    let tank = Entity::from_xyz_rad(
        xt.clone(),
        yt,
        vec![0.; xt.len()],
        vec![spacing / 2.; xt.len()],
    );
    let circle = Entity::from_xyz_rad(
        xc.clone(),
        yc,
        vec![0.; xc.len()],
        vec![spacing / 2.; xc.len()],
    );

    circle.write_vtk("zhang_circles.vtk");
    tank.write_vtk("zhang_tank.vtk");
}
// code:example_zhang_geometry ends here
