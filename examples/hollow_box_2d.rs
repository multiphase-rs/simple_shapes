extern crate simple_shapes;
extern crate vtkio;
use simple_shapes::hollow_box_2d;
use std::path::PathBuf;
use vtkio::export_ascii;
use vtkio::model::*;

fn main() {
    // dimension
    let spacing = 0.03;
    let (x1, y1) = hollow_box_2d(0., 1.0, spacing, 0., 1.0, spacing, 2, true);

    // println!("{}", step_no);
    let filename = format!("hollow_box_2d_outside.vtk");

    let mut pos = vec![];
    let mut radius = vec![];
    for i in 0..x1.len() {
        pos.push(x1[i]);
        pos.push(y1[i]);
        pos.push(0.);
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

    let spacing = 0.03;
    let (x1, y1) = hollow_box_2d(0., 1.0, spacing, 0., 1.0, spacing, 2, false);
    let filename = format!("hollow_box_2d_inside.vtk");

    let mut pos = vec![];
    let mut radius = vec![];
    for i in 0..x1.len() {
        pos.push(x1[i]);
        pos.push(y1[i]);
        pos.push(0.);
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
