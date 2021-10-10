// Internal module declarations.
mod web;

fn main() {
    let mut w_packet = web::rw::Writer::new(4);
    w_packet.write_int(&5_i32);

    println!("{:?}", w_packet.build());
}
