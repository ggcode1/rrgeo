#![feature(float_extras)]
extern crate kdtree;
extern crate csv;
extern crate rustc_serialize;

#[derive(RustcDecodable)]
struct Record {
    lat: f64,
    lon: f64,
    name: String,
    admin1: String,
    admin2: String,
    admin3: String
}

fn geodetic_in_ecef(geo_coords: (f32, f32)) -> (f32, f32, f32) {
    let a = 6378.137; // major axis in kms
    let e2 = 0.00669437999014;

    let lat = geo_coords.0;
    let lon = geo_coords.1;

    let lat_r = lat.to_radians();
    let lon_r = lon.to_radians();
    let normal = a / (1f32 - e2 * lat_r.sin().powi(2));

    let x = normal * lat_r.cos() * lon_r.cos();
    let y = normal * lat_r.cos() * lon_r.sin();
    let z = normal * (1f32 - e2) * lat.sin();
    //
    // return np.column_stack([x,y,z])
    (x, y, z)
}

fn main() {
    use kdtree::KdTree;
    use kdtree::ErrorKind;
    use kdtree::distance::squared_euclidean;

    let mut coords = Vec::new();
    let mut records = Vec::new();

    let dimensions = 2;
    let mut kdtree = KdTree::new(dimensions);

    let mut rdr = csv::Reader::from_file("cities.csv").unwrap();
    for record in rdr.decode() {
        let r: Record = record.unwrap();
        println!("({}, {}): {} {} {} {}", r.lat, r.lon, r.name, r.admin1, r.admin2, r.admin3);
        coords.push([r.lat, r.lon]);
        records.push(r);
    }

    for i in 0..coords.len() {
        kdtree.add(&coords[i], &records[i]);
    }
}
