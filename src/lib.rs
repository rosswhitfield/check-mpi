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

    mask_to_cpu_list(mask, size)
}

fn mask_to_cpu_list(mask: libc::cpu_set_t, size: libc::size_t) -> String {
    let mut cpu_list = String::new();

    let mut i = 0;
    loop {
        if unsafe { libc::CPU_ISSET(i, &mask) } {
            let mut run = 0;
            for j in i + 1..size {
                if unsafe { libc::CPU_ISSET(j, &mask) } {
                    run += 1;
                } else {
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

    #[test]
    fn test_mask_to_cpu_list() {
        let mut mask: libc::cpu_set_t = unsafe { std::mem::zeroed() };
        assert_eq!(mask_to_cpu_list(mask, 16), "");
        unsafe { libc::CPU_SET(0, &mut mask) };
        assert_eq!(mask_to_cpu_list(mask, 16), "0");
        unsafe { libc::CPU_SET(1, &mut mask) };
        assert_eq!(mask_to_cpu_list(mask, 16), "0,1");
        unsafe { libc::CPU_SET(2, &mut mask) };
        assert_eq!(mask_to_cpu_list(mask, 16), "0-2");

        unsafe { libc::CPU_ZERO(&mut mask) }
        unsafe { libc::CPU_SET(1, &mut mask) };
        unsafe { libc::CPU_SET(3, &mut mask) };
        unsafe { libc::CPU_SET(4, &mut mask) };
        unsafe { libc::CPU_SET(5, &mut mask) };
        unsafe { libc::CPU_SET(6, &mut mask) };
        unsafe { libc::CPU_SET(8, &mut mask) };
        unsafe { libc::CPU_SET(9, &mut mask) };
        unsafe { libc::CPU_SET(12, &mut mask) };
        unsafe { libc::CPU_SET(14, &mut mask) };
        unsafe { libc::CPU_SET(15, &mut mask) };
        assert_eq!(mask_to_cpu_list(mask, 16), "1,3-6,8,9,12,14,15");
    }
}
