use std::os::fd::{AsRawFd, FromRawFd};

use tester::BIBLE;

fn round_up_to_pagesize(n: usize) -> usize {
    let page_size: i64 = unsafe { ::tester::libc::sysconf(::tester::libc::_SC_PAGESIZE) };
    assert_ne!(page_size, -1);
    let page_size = page_size as usize;
    (n + page_size - 1) & !(page_size - 1)
}

fn helper(whole_buf: &[u8]) {
    let fd = unsafe { ::tester::libc::memfd_create(c"memfd-test".as_ptr(), ::tester::libc::MFD_CLOEXEC) };
    assert_ne!(fd, -1);
    // SAFETY: we just checked !
    let fd = unsafe { std::os::unix::io::OwnedFd::from_raw_fd(fd) };
    let mmap_size = round_up_to_pagesize(whole_buf.len() * 2);
    let mut buffer = vec![0; mmap_size].into_boxed_slice();
    assert_eq!(
        unsafe { ::tester::libc::write(fd.as_raw_fd(), buffer.as_ptr().cast(), mmap_size) },
        mmap_size as isize,
        "Failed to initialize memfd with zeroes"
    );
    assert_ne!(
        unsafe { ::tester::libc::lseek(fd.as_raw_fd(), 0, ::tester::libc::SEEK_SET) },
        -1
    );
    let ptr = unsafe {
        ::tester::libc::mmap(
            std::ptr::null_mut(),
            mmap_size,
            ::tester::libc::PROT_READ | ::tester::libc::PROT_WRITE,
            ::tester::libc::MAP_PRIVATE,
            fd.as_raw_fd(),
            0,
        )
    };
    assert!(!ptr.is_null(), "mmap failed -> please retry");
    let slice = unsafe { std::slice::from_raw_parts_mut(ptr.cast::<u8>(), mmap_size) };

    assert!(
        slice.iter().all(|s| *s == 0),
        "memfd isn't all set to zeroes ????!"
    );
    buffer.fill(255);

    assert_eq!(
        unsafe { ::tester::libc::write(fd.as_raw_fd(), whole_buf.as_ptr().cast(), whole_buf.len()) },
        whole_buf.len() as isize,
    );
    assert_ne!(
        unsafe { ::tester::libc::lseek(fd.as_raw_fd(), 0, ::tester::libc::SEEK_SET) },
        -1
    );
    assert_eq!(&slice[..whole_buf.len()], whole_buf);

    let ret = unsafe {
        tester::libasm::ft_read(fd.as_raw_fd(), buffer.as_mut_ptr().cast(), whole_buf.len())
    };
    assert_ne!(ret, -1);
    assert_eq!(ret as usize, whole_buf.len());
    assert_eq!(&buffer[..whole_buf.len()], whole_buf);

    unsafe { ::tester::libc::munmap(ptr, mmap_size) };
    drop(fd);
}

macro_rules! read_test {
    ($name:ident: $input:expr) => {
        #[test]
        fn $name() {
            helper($input);
        }
    };
}

read_test!(bible: &BIBLE);
read_test!(bible_half: &BIBLE[..BIBLE.len() / 2]);
read_test!(bible_1: &BIBLE[..1]);

read_test!(some_chars: b"hello I am a test !");
read_test!(no_chars: b"");

#[test]
fn invalid_fd() {
    let mut buf: [u8; 32] = [0; _];
    unsafe { ::tester::libc::__errno_location().write(0) };
    let ret = unsafe { tester::libasm::ft_read(10000, buf.as_mut_ptr().cast(), buf.len()) };
    let errno = unsafe { ::tester::libc::__errno_location().read() };
    assert_eq!(ret, -1);
    assert_eq!(errno, ::tester::libc::EBADF);
}

#[test]
fn again() {
    let mut pipes = [-1, -1];
    assert_ne!(
        unsafe { ::tester::libc::pipe2(pipes.as_mut_ptr(), ::tester::libc::O_NONBLOCK) },
        -1
    );
    pipes.iter().for_each(|&fd| unsafe {
        let flags = ::tester::libc::fcntl(fd, ::tester::libc::F_GETFL);
        assert_ne!(flags, -1);
        assert_ne!(::tester::libc::fcntl(fd, ::tester::libc::F_SETFL, flags | ::tester::libc::O_NONBLOCK), -1);
    });

    let mut buf: [u8; 32] = [0; _];
    unsafe { ::tester::libc::__errno_location().write(0) };
    let ret = unsafe { tester::libasm::ft_read(pipes[0], buf.as_mut_ptr().cast(), buf.len()) };
    let errno = unsafe { ::tester::libc::__errno_location().read() };
    unsafe { ::tester::libc::puts(::tester::libc::strerror(errno)) };
    assert_eq!(ret, -1);
    assert_eq!(errno, ::tester::libc::EAGAIN);

    pipes.iter().for_each(|&fd| unsafe {
        ::tester::libc::close(fd);
    });
}
