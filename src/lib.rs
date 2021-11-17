/// Returns the hostname of the system
///
/// This uses a call to libc::gethostname
pub fn get_hostname() -> String {
    let size = unsafe { libc::sysconf(libc::_SC_HOST_NAME_MAX) } as libc::size_t;

    let mut hostname = vec![0u8; size];
    unsafe {
        libc::gethostname(hostname.as_mut_ptr() as *mut libc::c_char, size);
    }

    String::from_iter(hostname.iter().filter(|&&e| e != 0_u8).map(|&e| e as char))
}

/// Return the core affinity of the current process
///
/// This uses a call to libc::sched_getaffinity
pub fn get_affinity() -> String {
    let mut mask: libc::cpu_set_t = unsafe { std::mem::zeroed() };

    unsafe {
        libc::sched_getaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &mut mask);
    }

    let size = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) } as libc::size_t;

    let mut cpu_list = String::new();

    let mut i = 0;
    loop {
        if unsafe { libc::CPU_ISSET(i, &mask) } {
            let mut run = 0;
            let mut j = i + 1;
            loop {
                if unsafe { libc::CPU_ISSET(j, &mask) } {
                    run += 1;
                } else {
                    break;
                }
                j += 1;
                if j == size {
                    break;
                }
            }
            if !cpu_list.is_empty() {
                cpu_list.push(',');
            }
            if run == 0 {
                cpu_list.push_str(&format!("{}", i));
            } else if run == 1 {
                cpu_list.push_str(&format!("{},{}", i, i + 1));
                i += 1;
            } else {
                cpu_list.push_str(&format!("{}-{}", i, i + run));
                i += run;
            }
        }
        i += 1;
        if i == size {
            break;
        }
    }

    cpu_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hostname() {
        assert!(!get_hostname().is_empty());
    }

    #[test]
    fn test_get_affinity() {
        let affinity = get_affinity();
        assert!(!affinity.is_empty());
        assert_eq!(&affinity[..1], "0");
    }
}
