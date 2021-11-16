pub fn get_hostname() -> String {
    let size = unsafe { libc::sysconf(libc::_SC_HOST_NAME_MAX) } as libc::size_t;
    let mut hostname = vec![0u8; size];
    unsafe {
        libc::gethostname(hostname.as_mut_ptr() as *mut libc::c_char, size);
    }

    String::from_iter(hostname.iter().map(|&e| e as char))
}

pub fn get_affinty() -> String {
    let mut mask: libc::cpu_set_t = unsafe { std::mem::zeroed() };

    unsafe {
        libc::sched_getaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &mut mask);
    }

    let size = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) } as libc::size_t;

    let mut cpu_list: Vec<String> = Vec::new();
    for i in 0..size {
        if unsafe { libc::CPU_ISSET(i, &mask) } {
            cpu_list.push(i.to_string());
        }
    }

    cpu_list.join(",")
}
