extern crate vtkio;
extern crate simple_shapes;
use simple_shapes::grid_arange_3d;
use vtkio::export_ascii;
use vtkio::model::*;
use std::path::PathBuf;

fn main() {
    // dimension
    let spacing = 0.03;
    let (x, y, z) = grid_arange_3d(0., 1.0, spacing, 0., 1.0, spacing, 0., 1.0, spacing);
    // println!("{}", step_no);
    let filename = format!("body.vtk");

    let mut pos = vec![];
    let mut radius = vec![];
    for i in 0..x.len() {
        pos.push(x[i]);
        pos.push(y[i]);
        pos.push(z[i]);
        radius.push(spacing / 2.);
    }

    let mut attributes = Attributes::new();
    attributes.point.push((
        "Radius".to_string(),
        Attribute::Scalars {
            num_comp: 1,
            lookup_table: None,
            data: radius.into(),
        },
    ));

    let data = DataSet::UnstructuredGrid {
        points: pos.into(),
        cells: Cells {
            num_cells: 0,
            vertices: vec![],
        },
        cell_types: vec![],
        data: attributes,
    };

    let vtk = Vtk {
        version: Version::new((4, 1)),
        title: String::from("Data"),
        data: data,
    };

    let _p = export_ascii(vtk, &PathBuf::from(&filename));
}
