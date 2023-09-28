const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // // part 1
    // let mut missiles = STARTING_MISSILES;
    // let ready = READY_AMOUNT;
    // println!("Firing {} of my {} missiles...", ready, missiles);

    // // part 2
    // missiles = missiles - ready;
    // println!("{} missiles left", missiles);

    // extra challenge
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}
