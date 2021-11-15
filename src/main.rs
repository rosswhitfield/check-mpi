use mpi::topology::Communicator;

fn main() {
    let rank = mpi::initialize().unwrap().world().rank();

    let size = unsafe { libc::sysconf(libc::_SC_HOST_NAME_MAX) } as libc::size_t;
    let mut hostname = vec![0u8; size];
    unsafe {
        libc::gethostname(hostname.as_mut_ptr() as *mut libc::c_char, size);
    }

    let hostname = String::from_iter(hostname.iter().map(|&e| e as char));

    let mut mask: libc::cpu_set_t = unsafe { std::mem::zeroed() };

    unsafe {
        libc::sched_getaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &mut mask);
    }

    let mut cpu_list: Vec<String> = Vec::new();
    for i in 0..libc::CPU_SETSIZE as usize {
        if unsafe { libc::CPU_ISSET(i, &mask) } {
            cpu_list.push(i.to_string());
        }
    }

    println!(
        "Hello from rank {}, on {}. (core affinity = {})",
        rank,
        hostname,
        cpu_list.join(",")
    );
}
