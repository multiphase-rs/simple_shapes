use crate::tank_2d;
pub use itertools_num::linspace;
use std::f64::consts::PI;

fn circle_2d(center: (f64, f64), radius: f64, spacing: f64) -> (Vec<f64>, Vec<f64>) {
    let dx = spacing;
    let mut x = vec![0.0];
    let mut y = vec![0.0];
    let mut r = spacing;
    let mut nt = 0;
    while r < radius {
        let nnew = (PI * r.powf(2.) / dx.powf(2.) + 0.5) as usize;
        let tomake = nnew - nt;
        let theta = linspace::<f64>(0., 2. * PI, tomake + 1).collect::<Vec<_>>();
        for t in 0..theta.len() - 1 {
            x.push(r * theta[t].cos());
            y.push(r * theta[t].sin());
        }
        nt = nnew;
        r = r + dx;
    }

    // let (x, y) = (t.ravel() for t in (x, y));

    x.iter_mut().for_each(|x| *x += center.0);
    y.iter_mut().for_each(|y| *y += center.1);
    return (x, y);
}

fn create_circle_only_boundary(
    center: (f64, f64),
    radius: f64,
    spacing: f64,
    layers: usize,
    closer: bool,
) -> (Vec<f64>, Vec<f64>) {
    let mut x = vec![];
    let mut y = vec![];

    for i in 1..layers + 1 {
        let r = radius - (i as f64 - 1.) * spacing - spacing / 2.;
        // number of points on the circumference
        let num = 2. * PI * r;

        let points = if closer {
            (num / (spacing / 2.)) as usize
        } else {
            (num / (spacing)) as usize
        };

        let theta = linspace::<f64>(0., 2. * PI, points).collect::<Vec<_>>();
        for t in 0..theta.len() - 1 {
            x.push(r * theta[t].cos());
            y.push(r * theta[t].sin());
        }
    }

    x.iter_mut().for_each(|x| *x += center.0);
    y.iter_mut().for_each(|y| *y += center.1);
    return (x, y);
}

/// Create stack of cylinders as in the benchmark of Zhang
pub fn create_cylinders_zhang(spacing: f64) -> (Vec<f64>, Vec<f64>, Vec<usize>) {
    // We follow three steps while creating the geometry
    // - Create bottom layer
    // - Create bottom second layer
    // - Create bottom third layer

    // ---------------------------------------
    // Create bottom layer
    // create a cylinder in 2d (that would be a circle),
    // let diameter = 0.01 + 4. * spacing; // in meters
    // let (xc1, yc1) = circle_2d(
    //     (diameter / 2. + spacing, diameter / 2. + spacing),
    //     diameter / 2.,
    //     spacing,
    // );

    let diameter = 0.01; // in meters
    let (xc1, yc1) = create_circle_only_boundary(
        (diameter / 2., diameter / 2.),
        diameter / 2.,
        spacing,
        2,
        true,
    );

    // get the number of particles in cylinder to compute the body id later
    let no_of_particles = xc1.len();

    let xc2: Vec<_> = xc1.iter().map(|&i| i + diameter).collect();
    let yc2 = yc1.clone();
    let xc3: Vec<_> = xc2.iter().map(|&i| i + diameter).collect();
    let yc3 = yc1.clone();
    let xc4: Vec<_> = xc3.iter().map(|&i| i + diameter).collect();
    let yc4 = yc1.clone();
    let xc5: Vec<_> = xc4.iter().map(|&i| i + diameter).collect();
    let yc5 = yc1.clone();
    let xc6: Vec<_> = xc5.iter().map(|&i| i + diameter).collect();
    let yc6 = yc1.clone();
    let x_layer_1 = [&xc1[..], &xc2[..], &xc3[..], &xc4[..], &xc5[..], &xc6[..]].concat();
    let y_layer_1 = [&yc1[..], &yc2[..], &yc3[..], &yc4[..], &yc5[..], &yc6[..]].concat();
    // ---------------------------------------

    // ---------------------------------------
    // Create second bottom layer

    // let (xc1, yc1) = circle_2d(
    //     (diameter + spacing, 1.5 * diameter + spacing),
    //     diameter / 2.,
    //     spacing,
    // );
    let (xc1, yc1) =
        create_circle_only_boundary((diameter, 1. * diameter + diameter / 2. - 2.5 * spacing), diameter / 2., spacing, 2, true);

    let xc2: Vec<_> = xc1.iter().map(|&i| i + diameter).collect();
    let yc2 = yc1.clone();
    let xc3: Vec<_> = xc2.iter().map(|&i| i + diameter).collect();
    let yc3 = yc1.clone();
    let xc4: Vec<_> = xc3.iter().map(|&i| i + diameter).collect();
    let yc4 = yc1.clone();
    let xc5: Vec<_> = xc4.iter().map(|&i| i + diameter).collect();
    let yc5 = yc1.clone();
    let x_layer_2 = [&xc1[..], &xc2[..], &xc3[..], &xc4[..], &xc5[..]].concat();
    let y_layer_2 = [&yc1[..], &yc2[..], &yc3[..], &yc4[..], &yc5[..]].concat();

    // ---------------------------------------
    // Create third bottom layer
    let x_layer_3 = x_layer_1.clone();
    let y_layer_3 = y_layer_1
        .iter()
        .map(|&i| i + 2. * diameter - 5. * spacing)
        .collect::<Vec<_>>();

    // ---------------------------------------
    // ---------------------------------------
    // Create fourth bottom layer
    let x_layer_4 = x_layer_2.clone();
    let y_layer_4 = y_layer_2
        .iter()
        .map(|&i| i + 2. * diameter - 5. * spacing)
        .collect::<Vec<_>>();

    // Create fifth bottom layer
    let x_layer_5 = x_layer_3.clone();
    let y_layer_5 = y_layer_3
        .iter()
        .map(|&i| i + 2. * diameter - 5. * spacing)
        .collect::<Vec<_>>();

    // Create sixth bottom layer
    let x_layer_6 = x_layer_4.clone();
    let y_layer_6 = y_layer_4
        .iter()
        .map(|&i| i + 2. * diameter - 5. * spacing)
        .collect::<Vec<_>>();

    let x_all = [
        &x_layer_1[..],
        &x_layer_2[..],
        &x_layer_3[..],
        &x_layer_4[..],
        &x_layer_5[..],
        &x_layer_6[..],
    ]
    .concat();
    let y_all = [
        &y_layer_1[..],
        &y_layer_2[..],
        &y_layer_3[..],
        &y_layer_4[..],
        &y_layer_5[..],
        &y_layer_6[..],
    ]
    .concat();

    // create body id of each cylinder
    let no_of_cylinders = 3 * 6 + 3 * 5;
    let mut b_id = vec![];

    for i in 0..no_of_cylinders {
        b_id.extend_from_slice(&vec![i; no_of_particles]);
    }

    (x_all, y_all, b_id)
}

/// Create geometry as in the benchmark of Zhang solid bodies
pub fn create_zhang_geometry(spacing: f64) -> (Vec<f64>, Vec<f64>, Vec<usize>, Vec<f64>, Vec<f64>) {
    // get the x, y and body id vectors
    let (xc, yc, bid) = create_cylinders_zhang(spacing);
    // create the tank
    let layers = 3;
    // create a tank with 26 cm length, 26 cm height
    let (xt, yt) = tank_2d(
        0.0,
        0.26 + spacing / 2.,
        spacing,
        0.0,
        0.26 + spacing / 2.,
        spacing,
        layers,
        true,
    );

    (xc, yc, bid, xt, yt)
}
