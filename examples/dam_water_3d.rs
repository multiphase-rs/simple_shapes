extern crate vtkio;
extern crate simple_shapes;

use simple_shapes::{grid_arange_3d, tank_3d, Entity};

fn main() {
    // dimension
    let spacing = 0.03;
    let (x, y, z) = grid_arange_3d(0., 1.0, spacing, 0., 1.0, spacing, 0., 1.0, spacing);
    let (xt, yt, zt) = tank_3d(-1.1, 3.0, spacing, -1.1, 4.0, spacing, -1.1, 4.0, spacing, 2);

    let fluid = Entity::from_xyz_rad(x.clone(), y, z, vec![spacing/2.; x.len()]);
    let tank = Entity::from_xyz_rad(xt.clone(), yt, zt, vec![spacing/2.; xt.len()]);

    fluid.write_vtk("fluid_3d.vtk");
    tank.write_vtk("tank_3d.vtk");
}
