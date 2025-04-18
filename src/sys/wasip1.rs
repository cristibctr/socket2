use std::io::{self, IoSlice, IoSliceMut};
use std::mem::MaybeUninit;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::time::Duration;
use std::marker::PhantomData;

use crate::{Domain, Protocol, SockAddr, Type, TcpKeepalive};

// Define some constants to match the expected API
pub(crate) type c_int = i32;
pub(crate) type sa_family_t = u16;
pub(crate) type socklen_t = u32;
pub(crate) type Bool = c_int;

// socket types
pub(crate) const SOCK_STREAM: c_int = 1;
pub(crate) const SOCK_DGRAM: c_int = 2;

// Socket domains/families
pub(crate) const AF_INET: c_int = 2;
pub(crate) const AF_INET6: c_int = 10;
pub(crate) const AF_UNIX: c_int = 1;

// Protocol families
pub(crate) const IPPROTO_IP: c_int = 0;
pub(crate) const IPPROTO_IPV6: c_int = 41;
pub(crate) const IPPROTO_TCP: c_int = 6;
pub(crate) const IPPROTO_UDP: c_int = 17;
pub(crate) const IPPROTO_ICMP: c_int = 1;
pub(crate) const IPPROTO_ICMPV6: c_int = 58;

// Socket options
pub(crate) const SOL_SOCKET: c_int = 1;
pub(crate) const SO_BROADCAST: c_int = 6;
pub(crate) const SO_ERROR: c_int = 4;
pub(crate) const SO_KEEPALIVE: c_int = 9;
pub(crate) const SO_LINGER: c_int = 13;
pub(crate) const SO_RCVBUF: c_int = 8;
pub(crate) const SO_RCVTIMEO: c_int = 20;
pub(crate) const SO_REUSEADDR: c_int = 2;
pub(crate) const SO_SNDBUF: c_int = 7;
pub(crate) const SO_SNDTIMEO: c_int = 21;
pub(crate) const SO_TYPE: c_int = 3;
pub(crate) const TCP_NODELAY: c_int = 1;
pub(crate) const MSG_TRUNC: c_int = 0x20;

// IP specific options
pub(crate) const IP_TTL: c_int = 2;
pub(crate) const IPV6_UNICAST_HOPS: c_int = 16;

#[cfg(not(feature = "all"))]
pub(crate) const SO_RCVLOWAT: c_int = 0; 
#[cfg(not(feature = "all"))]
pub(crate) const SO_SNDLOWAT: c_int = 0; 

// Socket address structures
#[repr(C)]
pub(crate) struct sockaddr {
    pub(crate) sa_family: sa_family_t,
    pub(crate) sa_data: [u8; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct in_addr {
    pub(crate) s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct in6_addr {
    pub(crate) s6_addr: [u8; 16],
}

#[repr(C)]
pub(crate) struct sockaddr_in {
    pub(crate) sin_family: sa_family_t,
    pub(crate) sin_port: u16,
    pub(crate) sin_addr: in_addr,
    pub(crate) sin_zero: [u8; 8],
}

#[repr(C)]
pub(crate) struct sockaddr_in6 {
    pub(crate) sin6_family: sa_family_t,
    pub(crate) sin6_port: u16,
    pub(crate) sin6_flowinfo: u32,
    pub(crate) sin6_addr: in6_addr,
    pub(crate) sin6_scope_id: u32,
}

#[repr(C)]
#[derive(Clone)]
pub(crate) struct sockaddr_storage {
    pub(crate) ss_family: sa_family_t,
    pub(crate) __ss_padding: [u8; 128 - 2],
}

// Unix domain socket structure (stub for WASI)
#[repr(C)]
pub(crate) struct sockaddr_un {
    pub(crate) sun_family: sa_family_t,
    pub(crate) sun_path: [u8; 108],
}

// We represent the socket as a raw file descriptor
pub(crate) type Socket = RawFd;

fn unsupported_error() -> io::Error {
    io::Error::new(io::ErrorKind::Unsupported, "WASI P1 sockets not supported")
}

pub(crate) fn socket(_family: c_int, _ty: c_int, _protocol: c_int) -> io::Result<Socket> {
    Err(unsupported_error())
}

pub(crate) fn bind(_socket: Socket, _addr: &SockAddr) -> io::Result<()> {
    Err(unsupported_error())
}

pub(crate) fn connect(_socket: Socket, _addr: &SockAddr) -> io::Result<()> {
    Err(unsupported_error())
}

pub(crate) fn listen(_socket: Socket, _backlog: c_int) -> io::Result<()> {
    Err(unsupported_error())
}

pub(crate) fn accept(_socket: Socket) -> io::Result<(Socket, SockAddr)> {
    Err(unsupported_error())
}

pub(crate) fn getsockname(_socket: Socket) -> io::Result<SockAddr> {
    Err(unsupported_error())
}

pub(crate) fn getpeername(_socket: Socket) -> io::Result<SockAddr> {
    Err(unsupported_error())
}

pub(crate) fn poll_connect(_socket: &crate::Socket, _timeout: Duration) -> io::Result<()> {
    Err(unsupported_error())
}

pub(crate) fn recv(
    _socket: Socket,
    _buf: &mut [MaybeUninit<u8>],
    _flags: c_int,
) -> io::Result<usize> {
    Err(unsupported_error())
}

pub(crate) fn recv_from(
    _socket: Socket,
    _buf: &mut [MaybeUninit<u8>],
    _flags: c_int,
) -> io::Result<(usize, SockAddr)> {
    Err(unsupported_error())
}

pub(crate) fn send(_socket: Socket, _buf: &[u8], _flags: c_int) -> io::Result<usize> {
    Err(unsupported_error())
}

pub(crate) fn send_to(
    _socket: Socket,
    _buf: &[u8],
    _addr: &SockAddr,
    _flags: c_int,
) -> io::Result<usize> {
    Err(unsupported_error())
}

pub(crate) fn set_nonblocking(_socket: Socket, _nonblocking: bool) -> io::Result<()> {
    Err(unsupported_error())
}

pub(crate) fn shutdown(_socket: Socket, _how: std::net::Shutdown) -> io::Result<()> {
    Err(unsupported_error())
}

pub(crate) unsafe fn setsockopt<T>(
    _socket: Socket,
    _level: c_int,
    _optname: c_int,
    _optval: T,
) -> io::Result<()> {
    Err(unsupported_error())
}

pub(crate) unsafe fn getsockopt<T: Copy>(
    _socket: Socket,
    _level: c_int,
    _optname: c_int,
) -> io::Result<T> {
    Err(unsupported_error())
}

// Define msghdr struct for compatibility
pub(crate) struct msghdr {
    msg_name: *mut std::ffi::c_void,
    msg_namelen: socklen_t,
    msg_iov: *mut IoSlice<'static>,
    msg_iovlen: usize,
    msg_control: *mut std::ffi::c_void,
    msg_controllen: usize,
    msg_flags: c_int,
}

// Define MaybeUninitSlice
#[derive(Debug)]
pub struct MaybeUninitSlice<'a>(*mut IoSliceMut<'static>, PhantomData<&'a mut [MaybeUninit<u8>]>);

impl<'a> MaybeUninitSlice<'a> {
    pub(crate) fn new(_buf: &'a mut [MaybeUninit<u8>]) -> MaybeUninitSlice<'a> {
        MaybeUninitSlice(std::ptr::null_mut(), PhantomData)
    }
    pub(crate) fn as_slice(&self) -> &[MaybeUninit<u8>] {
        &[]
    }
    pub(crate) fn as_mut_slice(&mut self) -> &mut [MaybeUninit<u8>] {
        &mut []
    }
}

pub(crate) fn recv_vectored(
    _socket: Socket,
    _bufs: &mut [MaybeUninitSlice<'_>],
    _flags: c_int,
) -> io::Result<usize> {
    Err(unsupported_error())
}

pub(crate) fn send_vectored(
    _socket: Socket,
    _bufs: &[IoSlice<'_>],
    _flags: c_int,
) -> io::Result<usize> {
    Err(unsupported_error())
}

pub(crate) fn recvmsg(
    _socket: Socket,
    _hdr: &mut msghdr,
    _flags: c_int,
) -> io::Result<usize> {
    Err(unsupported_error())
}

pub(crate) fn sendmsg(
    _socket: Socket,
    _hdr: &msghdr,
    _flags: c_int,
) -> io::Result<usize> {
    Err(unsupported_error())
}

pub(crate) fn set_msghdr_name(_hdr: &mut msghdr, _addr: &SockAddr) {
    // No-op
}

pub(crate) fn set_msghdr_iov(
    _hdr: &mut msghdr,
    _iov: *mut IoSlice<'static>,
    _len: usize,
) {
    // No-op
}

pub(crate) fn set_msghdr_control(
    _hdr: &mut msghdr,
    _control: *mut std::ffi::c_void,
    _len: usize,
) {
    // No-op
}

pub(crate) fn set_msghdr_flags(_hdr: &mut msghdr, _flags: c_int) {
    // No-op
}

// Special handling for RecvFlags since it's cfg'd out on WASI
pub(crate) struct WasiRecvFlags(c_int);

pub(crate) fn msghdr_flags(_hdr: &msghdr) -> WasiRecvFlags {
    WasiRecvFlags(0)
}

pub(crate) fn msghdr_control_len(_hdr: &msghdr) -> usize {
    0
}

pub(crate) const fn to_in_addr(addr: &Ipv4Addr) -> in_addr {
    in_addr {
        s_addr: u32::from_ne_bytes(addr.octets()),
    }
}

pub(crate) const fn to_in6_addr(addr: &Ipv6Addr) -> in6_addr {
    in6_addr {
        s6_addr: addr.octets(),
    }
}

pub(crate) fn from_in_addr(addr: in_addr) -> Ipv4Addr {
    Ipv4Addr::from(addr.s_addr.to_ne_bytes())
}

pub(crate) fn from_in6_addr(addr: in6_addr) -> Ipv6Addr {
    Ipv6Addr::from(addr.s6_addr)
}

pub(crate) fn into_duration(timeval: libc::timeval) -> Duration {
    Duration::new(timeval.tv_sec as u64, timeval.tv_usec as u32 * 1000)
}

pub(crate) fn into_timeval(duration: Option<Duration>) -> libc::timeval {
    match duration {
        Some(duration) => libc::timeval {
            tv_sec: std::cmp::min(duration.as_secs(), libc::time_t::MAX as u64) as libc::time_t,
            tv_usec: duration.subsec_micros() as libc::suseconds_t,
        },
        None => libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    }
}

// TCP_KEEPALIVE related functions
pub(crate) fn set_reuseaddr(_socket: Socket, _reuseaddr: bool) -> io::Result<()> {
    Err(unsupported_error())
}

pub(crate) fn set_linger(_socket: Socket, _dur: Option<Duration>) -> io::Result<()> {
    Err(unsupported_error())
}

pub(crate) fn set_recv_timeout(_socket: Socket, _dur: Option<Duration>) -> io::Result<()> {
    Err(unsupported_error())
}

pub(crate) fn set_send_timeout(_socket: Socket, _dur: Option<Duration>) -> io::Result<()> {
    Err(unsupported_error())
}

pub(crate) fn set_keepalive(_socket: Socket, _keepalive: bool) -> io::Result<()> {
    Err(unsupported_error())
}

pub(crate) fn set_tcp_keepalive(
    _socket: Socket,
    _keepalive: &TcpKeepalive,
) -> io::Result<()> {
    Err(unsupported_error())
}

// Socket utilities
pub(crate) fn socket_addr_is_unnamed(_addr: &sockaddr_un) -> bool {
    false
}

pub(crate) fn socket_addr_pathname(_addr: &sockaddr_un) -> Option<&[u8]> {
    None
}

pub(crate) fn socket_addr_abstract_namespace(_addr: &sockaddr_un) -> Option<&[u8]> {
    None
}

pub(crate) fn socket_addr_unspecified(_addr: &sockaddr_un) -> bool {
    false
}

pub(crate) fn socket_as_raw(_socket: &std::net::TcpStream) -> RawFd {
    -1
}

pub(crate) fn socket_into_raw(_socket: std::net::TcpStream) -> RawFd {
    -1
}

// Socket from/to raw conversions
pub(crate) fn socket_from_raw(raw: RawFd) -> crate::socket::Inner {
    unsafe { crate::socket::Inner::from_raw_fd(raw) }
}