





use std::io::{self, IoSlice, IoSliceMut};
use std::mem::MaybeUninit;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::time::Duration;

use crate::{Domain, Protocol, SockAddr, Type};

pub(crate) use libc::{
    c_int, sa_family_t, socklen_t, AF_INET, AF_INET6, IPPROTO_ICMP, IPPROTO_ICMPV6, IPPROTO_TCP,
    IPPROTO_UDP, MSG_TRUNC, SOCK_DGRAM, SOCK_STREAM, SOL_SOCKET, SO_BROADCAST, SO_ERROR,
    SO_KEEPALIVE, SO_LINGER, SO_RCVBUF, SO_RCVTIMEO, SO_REUSEADDR, SO_SNDBUF, SO_SNDTIMEO,
    SO_TYPE, TCP_NODELAY,
};


#[cfg(not(feature = "all"))]
pub(crate) const SO_RCVLOWAT: c_int = 0; 
#[cfg(not(feature = "all"))]
pub(crate) const SO_SNDLOWAT: c_int = 0; 


#[derive(Debug)]
pub struct Socket(RawFd);

impl Socket {
    pub(crate) fn new(
        _domain: Domain,
        _type_: Type,
        _protocol: Option<Protocol>,
    ) -> io::Result<Socket> {
        Err(unsupported_error())
    }
}

impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl IntoRawFd for Socket {
    fn into_raw_fd(self) -> RawFd {
        let fd = self.0;
        std::mem::forget(self);
        fd
    }
}

impl FromRawFd for Socket {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Socket(fd)
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        
        
    }
}


fn unsupported_error() -> io::Error {
    io::Error::new(io::ErrorKind::Unsupported, "WASI P1 sockets not supported")
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


pub(crate) type msghdr = libc::msghdr; 

#[derive(Debug)]
pub struct MaybeUninitSlice<'a>(*mut libc::iovec, PhantomData<&'a mut [MaybeUninit<u8>]>);
use std::marker::PhantomData;

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
    
}

pub(crate) fn set_msghdr_iov(
    _hdr: &mut msghdr,
    _iov: *mut libc::iovec,
    _len: usize,
) {
    
}

pub(crate) fn set_msghdr_control(
    _hdr: &mut msghdr,
    _control: *mut libc::c_void,
    _len: usize,
) {
    
}

pub(crate) fn set_msghdr_flags(_hdr: &mut msghdr, _flags: c_int) {
    
}

pub(crate) fn msghdr_flags(_hdr: &msghdr) -> crate::RecvFlags {
    crate::RecvFlags(0) 
}

pub(crate) fn msghdr_control_len(_hdr: &msghdr) -> usize {
    0 
}


pub(crate) use libc::{in6_addr, in_addr}; 

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
    _time: Option<Duration>,
    _interval: Option<Duration>,
    _retries: Option<u32>,
) -> io::Result<()> {
    Err(unsupported_error())
}


pub(crate) fn nix_err_to_io_err(_err: nix::Error) -> io::Error {
    unsupported_error()
}


pub(crate) fn socket_addr_is_unnamed(_addr: &libc::sockaddr_un) -> bool {
    false 
}

pub(crate) fn socket_addr_pathname(_addr: &libc::sockaddr_un) -> Option<&[u8]> {
    None 
}

pub(crate) fn socket_addr_abstract_namespace(_addr: &libc::sockaddr_un) -> Option<&[u8]> {
    None 
}

pub(crate) fn socket_addr_unspecified(_addr: &libc::sockaddr_un) -> bool {
    false 
}

pub(crate) fn socket_as_raw(_socket: &std::net::TcpStream) -> RawFd {
    
    -1 
}

pub(crate) fn socket_into_raw(_socket: std::net::TcpStream) -> RawFd {
    
    -1 
}
