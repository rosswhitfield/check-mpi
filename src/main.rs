use mpi::topology::Communicator;

fn main() {
    let rank = mpi::initialize().unwrap().world().rank();

    println!(
        "Hello from rank {}, on {}. (core affinity = {})",
        rank,
        checkmpi::get_hostname(),
        checkmpi::get_affinity()
    );
}
