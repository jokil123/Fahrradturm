use tower_controller_rs_v2::util::generate_n_dimensional_coords;

fn main() {
    let loc = generate_n_dimensional_coords(&vec![10, 1, 1]);

    println!("{:#?}", loc);
    println!("{:#?}", loc.len());
}
