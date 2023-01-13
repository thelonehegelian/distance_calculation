fn main() {
    let EARTH_RADIUS_KM = 6371.0_f64;
    let EARTH_RADIUS_MI = 3959.0_f64;
    // brussels longitude and latitude in degrees tuple
    let brussels = (4.3517_f64, 50.8503_f64);
    // london longitude and latitude in degrees tuple
    let london = (-0.1278_f64, 51.5074_f64);
    // lattitude in radians
    let brussels_lat = brussels.1.to_radians();
    let london_lat = london.1.to_radians();
    // longitude in radians
    let brussels_lon = brussels.0.to_radians();
    let london_lon = london.0.to_radians();

    // delta longitude in radians
    let delta_lon = london_lon - brussels_lon;
    // delta latitude in radians
    let delta_lat = london_lat - brussels_lat;
    // central inner angle
    let central_inner_angle = (delta_lat / 2.0_f64).sin().powi(2)
        + brussels_lat.cos() * london_lat.cos() * (delta_lon / 2.0_f64).sin().powi(2);
    // central angle
    let central_angle = 2.0_f64 * central_inner_angle.sqrt().asin();
    // distance in km
    let distance_km = EARTH_RADIUS_KM * central_angle;
    println!("Distance between Brussels and London is {} km", distance_km);
}
