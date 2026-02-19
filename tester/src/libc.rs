pub use ::std::ffi::{c_char, c_int, c_long, c_uint, c_void};
pub type size_t = usize;
pub type ssize_t = isize;
pub type off_t = i32;

extern "C" {
    pub fn __errno_location() -> *mut c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    pub fn free(p: *mut c_void);
    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    pub fn mmap(
        addr: *mut c_void,
        len: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    pub fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    pub fn pipe2(fds: *mut c_int, flags: c_int) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strerror(n: c_int) -> *mut c_char;
    pub fn sysconf(name: c_int) -> c_long;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
}

pub const EAGAIN: c_int = 11;
pub const EBADF: c_int = 9;
pub const EBADFD: c_int = 77;
pub const ENOSPC: c_int = 28;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const MAP_PRIVATE: c_int = 0x0002;
pub const MFD_CLOEXEC: c_uint = 0x0001;
pub const O_NONBLOCK: c_int = 2048;
pub const PROT_READ: c_int = 1;
pub const PROT_WRITE: c_int = 2;
pub const SEEK_SET: c_int = 0;
pub const _SC_PAGESIZE: c_int = 30;
