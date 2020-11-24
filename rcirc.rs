#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types, main,
           ptr_wrapping_offset_from, register_tool)]

use std::env;
use std::process;
use std::collections::HashSet;


extern "C" {
    pub type lws_sequencer;
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ssl_ctx_st;
    pub type lws;
    pub type lws_context;
    pub type lws_vhost;
    pub type lws_attach_item;
    pub type json_object;
    #[no_mangle]
    fn json_read(data: *mut libc::c_void, object: *mut json_object,
                 fmt: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn json_create(data: *mut libc::c_void, fmt: *const libc::c_char, _: ...)
     -> *mut json_object;
    #[no_mangle]
    fn reactions2string(jo: *mut json_object) -> *mut libc::c_char;
    #[no_mangle]
    fn logg(lvl: libc::c_short, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn logg_setmask(mask: libc::c_int);
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn lws_callback_on_writable(wsi: *mut lws) -> libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
    #[no_mangle]
    fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
              __shortopts: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int,
              __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lws_get_protocol(wsi: *mut lws) -> *const lws_protocols;
    #[no_mangle]
    fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t)
     -> libc::c_int;
    #[no_mangle]
    fn lws_service_fd(context: *mut lws_context, pollfd: *mut pollfd)
     -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn tsearch(__key: *const libc::c_void, __rootp: *mut *mut libc::c_void,
               __compar: __compar_fn_t) -> *mut libc::c_void;
    #[no_mangle]
    fn tfind(__key: *const libc::c_void, __rootp: *const *mut libc::c_void,
             __compar: __compar_fn_t) -> *mut libc::c_void;
    #[no_mangle]
    fn send(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t,
            __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn recv(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
            __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn json_object_put(obj: *mut json_object) -> libc::c_int;
    #[no_mangle]
    fn tdestroy(__root: *mut libc::c_void, __freefct: __free_fn_t);
    #[no_mangle]
    fn lws_timed_callback_vh_protocol(vh: *mut lws_vhost,
                                      prot: *const lws_protocols,
                                      reason: libc::c_int, secs: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn lws_client_connect_via_info(ccinfo: *const lws_client_connect_info)
     -> *mut lws;
    #[no_mangle]
    fn lws_callback_http_dummy(wsi: *mut lws, reason: lws_callback_reasons,
                               user: *mut libc::c_void,
                               in_0: *mut libc::c_void, len: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn lws_get_socket_fd(wsi: *mut lws) -> lws_sockfd_type;
    #[no_mangle]
    fn json_object_to_json_string(obj: *mut json_object)
     -> *const libc::c_char;
    #[no_mangle]
    fn lws_write(wsi: *mut lws, buf: *mut libc::c_uchar, len: size_t,
                 protocol: lws_write_protocol) -> libc::c_int;
    #[no_mangle]
    fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> libc::c_int;
    #[no_mangle]
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn json_tokener_parse(str: *const libc::c_char) -> *mut json_object;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
              __addr_len: *mut socklen_t) -> libc::c_int;
    #[no_mangle]
    fn lws_set_log_level(level: libc::c_int,
                         log_emit_function:
                             Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _:
                                                             *const libc::c_char)
                                        -> ()>);
    #[no_mangle]
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn lws_get_context(wsi: *const lws) -> *mut lws_context;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vasprintf(__ptr: *mut *mut libc::c_char, __f: *const libc::c_char,
                 __arg: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    fn json_object_object_add(obj: *mut json_object, key: *const libc::c_char,
                              val: *mut json_object) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn json_object_object_get(obj: *const json_object,
                              key: *const libc::c_char) -> *mut json_object;
    #[no_mangle]
    fn lws_get_opaque_user_data(wsi: *const lws) -> *mut libc::c_void;
    #[no_mangle]
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn stpncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn json_object_array_length(obj: *const json_object) -> size_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn json_object_array_get_idx(obj: *const json_object, idx: size_t)
     -> *mut json_object;
    #[no_mangle]
    fn getaddrinfo(__name: *const libc::c_char,
                   __service: *const libc::c_char, __req: *const addrinfo,
                   __pai: *mut *mut addrinfo) -> libc::c_int;
    #[no_mangle]
    fn freeaddrinfo(__ai: *mut addrinfo);
    #[no_mangle]
    fn lws_create_context(info: *const lws_context_creation_info)
     -> *mut lws_context;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn lws_context_destroy(context: *mut lws_context);
    #[no_mangle]
    fn json_object_new_string(s: *const libc::c_char) -> *mut json_object;
    #[no_mangle]
    fn lws_get_vhost(wsi: *mut lws) -> *mut lws_vhost;
    #[no_mangle]
    fn json_object_get_string(obj: *mut json_object) -> *const libc::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int64_t = __int64_t;
pub type pthread_t = libc::c_ulong;
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type socklen_t = __socklen_t;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed = 2;
pub const SHUT_WR: C2RustUnnamed = 1;
pub const SHUT_RD: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type SSL_CTX = ssl_ctx_st;
pub type lws_sockfd_type = libc::c_int;
pub type lws_filefd_type = libc::c_int;
pub type lws_usec_t = int64_t;
pub type lws_filepos_t = libc::c_ulonglong;
pub type lws_fileofs_t = libc::c_longlong;
pub type lws_fop_flags_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_extension {
    pub name: *const libc::c_char,
    pub callback: Option<lws_extension_callback_function>,
    pub client_offer: *const libc::c_char,
}
pub type lws_extension_callback_function
    =
    unsafe extern "C" fn(_: *mut lws_context, _: *const lws_extension,
                         _: *mut lws, _: lws_extension_callback_reasons,
                         _: *mut libc::c_void, _: *mut libc::c_void,
                         _: size_t) -> libc::c_int;
pub type lws_extension_callback_reasons = libc::c_uint;
pub const LWS_EXT_CB_NAMED_OPTION_SET: lws_extension_callback_reasons = 26;
pub const LWS_EXT_CB_OPTION_CONFIRM: lws_extension_callback_reasons = 25;
pub const LWS_EXT_CB_OPTION_SET: lws_extension_callback_reasons = 24;
pub const LWS_EXT_CB_OPTION_DEFAULT: lws_extension_callback_reasons = 23;
pub const LWS_EXT_CB_PAYLOAD_RX: lws_extension_callback_reasons = 22;
pub const LWS_EXT_CB_PAYLOAD_TX: lws_extension_callback_reasons = 21;
pub const LWS_EXT_CB_PACKET_TX_PRESEND: lws_extension_callback_reasons = 12;
pub const LWS_EXT_CB_DESTROY: lws_extension_callback_reasons = 8;
pub const LWS_EXT_CB_CLIENT_CONSTRUCT: lws_extension_callback_reasons = 5;
pub const LWS_EXT_CB_CONSTRUCT: lws_extension_callback_reasons = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_token_limits {
    pub token_limit: [libc::c_ushort; 95],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_protocols {
    pub name: *const libc::c_char,
    pub callback: Option<lws_callback_function>,
    pub per_session_data_size: size_t,
    pub rx_buffer_size: size_t,
    pub id: libc::c_uint,
    pub user: *mut libc::c_void,
    pub tx_packet_size: size_t,
}
pub type lws_callback_function
    =
    unsafe extern "C" fn(_: *mut lws, _: lws_callback_reasons,
                         _: *mut libc::c_void, _: *mut libc::c_void,
                         _: size_t) -> libc::c_int;
pub type lws_callback_reasons = libc::c_uint;
pub const LWS_CALLBACK_USER: lws_callback_reasons = 1000;
pub const LWS_CALLBACK_MQTT_RESEND: lws_callback_reasons = 210;
pub const LWS_CALLBACK_MQTT_ACK: lws_callback_reasons = 209;
pub const LWS_CALLBACK_MQTT_CLIENT_CLOSED: lws_callback_reasons = 208;
pub const LWS_CALLBACK_MQTT_DROP_PROTOCOL: lws_callback_reasons = 207;
pub const LWS_CALLBACK_MQTT_UNSUBSCRIBED: lws_callback_reasons = 206;
pub const LWS_CALLBACK_MQTT_CLIENT_RX: lws_callback_reasons = 205;
pub const LWS_CALLBACK_MQTT_CLIENT_WRITEABLE: lws_callback_reasons = 204;
pub const LWS_CALLBACK_MQTT_SUBSCRIBED: lws_callback_reasons = 203;
pub const LWS_CALLBACK_MQTT_CLIENT_ESTABLISHED: lws_callback_reasons = 202;
pub const LWS_CALLBACK_MQTT_IDLE: lws_callback_reasons = 201;
pub const LWS_CALLBACK_MQTT_NEW_CLIENT_INSTANTIATED: lws_callback_reasons =
    200;
pub const LWS_CALLBACK_VHOST_CERT_UPDATE: lws_callback_reasons = 74;
pub const LWS_CALLBACK_VHOST_CERT_AGING: lws_callback_reasons = 72;
pub const LWS_CALLBACK_CHILD_CLOSING: lws_callback_reasons = 69;
pub const LWS_CALLBACK_EVENT_WAIT_CANCELLED: lws_callback_reasons = 71;
pub const LWS_CALLBACK_TIMER: lws_callback_reasons = 73;
pub const LWS_CALLBACK_RAW_FILE_DROP_PROTOCOL: lws_callback_reasons = 84;
pub const LWS_CALLBACK_RAW_FILE_BIND_PROTOCOL: lws_callback_reasons = 83;
pub const LWS_CALLBACK_RAW_CLOSE_FILE: lws_callback_reasons = 66;
pub const LWS_CALLBACK_RAW_WRITEABLE_FILE: lws_callback_reasons = 65;
pub const LWS_CALLBACK_RAW_RX_FILE: lws_callback_reasons = 64;
pub const LWS_CALLBACK_RAW_ADOPT_FILE: lws_callback_reasons = 63;
pub const LWS_CALLBACK_RAW_SKT_DROP_PROTOCOL: lws_callback_reasons = 82;
pub const LWS_CALLBACK_RAW_SKT_BIND_PROTOCOL: lws_callback_reasons = 81;
pub const LWS_CALLBACK_RAW_CONNECTED: lws_callback_reasons = 101;
pub const LWS_CALLBACK_RAW_ADOPT: lws_callback_reasons = 62;
pub const LWS_CALLBACK_RAW_WRITEABLE: lws_callback_reasons = 61;
pub const LWS_CALLBACK_RAW_CLOSE: lws_callback_reasons = 60;
pub const LWS_CALLBACK_RAW_RX: lws_callback_reasons = 59;
pub const LWS_CALLBACK_RAW_PROXY_SRV_DROP_PROTOCOL: lws_callback_reasons =
    100;
pub const LWS_CALLBACK_RAW_PROXY_CLI_DROP_PROTOCOL: lws_callback_reasons = 99;
pub const LWS_CALLBACK_RAW_PROXY_SRV_BIND_PROTOCOL: lws_callback_reasons = 98;
pub const LWS_CALLBACK_RAW_PROXY_CLI_BIND_PROTOCOL: lws_callback_reasons = 97;
pub const LWS_CALLBACK_RAW_PROXY_SRV_ADOPT: lws_callback_reasons = 96;
pub const LWS_CALLBACK_RAW_PROXY_CLI_ADOPT: lws_callback_reasons = 95;
pub const LWS_CALLBACK_RAW_PROXY_SRV_WRITEABLE: lws_callback_reasons = 94;
pub const LWS_CALLBACK_RAW_PROXY_CLI_WRITEABLE: lws_callback_reasons = 93;
pub const LWS_CALLBACK_RAW_PROXY_SRV_CLOSE: lws_callback_reasons = 92;
pub const LWS_CALLBACK_RAW_PROXY_CLI_CLOSE: lws_callback_reasons = 91;
pub const LWS_CALLBACK_RAW_PROXY_SRV_RX: lws_callback_reasons = 90;
pub const LWS_CALLBACK_RAW_PROXY_CLI_RX: lws_callback_reasons = 89;
pub const LWS_CALLBACK_HTTP_PMO: lws_callback_reasons = 56;
pub const LWS_CALLBACK_GS_EVENT: lws_callback_reasons = 55;
pub const LWS_CALLBACK_SESSION_INFO: lws_callback_reasons = 54;
pub const LWS_CALLBACK_CGI_PROCESS_ATTACH: lws_callback_reasons = 70;
pub const LWS_CALLBACK_CGI_STDIN_COMPLETED: lws_callback_reasons = 43;
pub const LWS_CALLBACK_CGI_STDIN_DATA: lws_callback_reasons = 42;
pub const LWS_CALLBACK_CGI_TERMINATED: lws_callback_reasons = 41;
pub const LWS_CALLBACK_CGI: lws_callback_reasons = 40;
pub const LWS_CALLBACK_UNLOCK_POLL: lws_callback_reasons = 36;
pub const LWS_CALLBACK_LOCK_POLL: lws_callback_reasons = 35;
pub const LWS_CALLBACK_CHANGE_MODE_POLL_FD: lws_callback_reasons = 34;
pub const LWS_CALLBACK_DEL_POLL_FD: lws_callback_reasons = 33;
pub const LWS_CALLBACK_ADD_POLL_FD: lws_callback_reasons = 32;
pub const LWS_CALLBACK_GET_THREAD_ID: lws_callback_reasons = 31;
pub const LWS_CALLBACK_WS_CLIENT_DROP_PROTOCOL: lws_callback_reasons = 80;
pub const LWS_CALLBACK_WS_CLIENT_BIND_PROTOCOL: lws_callback_reasons = 79;
pub const LWS_CALLBACK_FILTER_NETWORK_CONNECTION: lws_callback_reasons = 17;
pub const LWS_CALLBACK_WS_EXT_DEFAULTS: lws_callback_reasons = 39;
pub const LWS_CALLBACK_CLIENT_CONFIRM_EXTENSION_SUPPORTED:
          lws_callback_reasons =
    26;
pub const LWS_CALLBACK_CLIENT_WRITEABLE: lws_callback_reasons = 10;
pub const LWS_CALLBACK_CLIENT_RECEIVE_PONG: lws_callback_reasons = 9;
pub const LWS_CALLBACK_CLIENT_RECEIVE: lws_callback_reasons = 8;
pub const LWS_CALLBACK_CLIENT_APPEND_HANDSHAKE_HEADER: lws_callback_reasons =
    24;
pub const LWS_CALLBACK_CLIENT_CLOSED: lws_callback_reasons = 75;
pub const LWS_CALLBACK_CLIENT_ESTABLISHED: lws_callback_reasons = 3;
pub const LWS_CALLBACK_CLIENT_FILTER_PRE_ESTABLISH: lws_callback_reasons = 2;
pub const LWS_CALLBACK_CLIENT_CONNECTION_ERROR: lws_callback_reasons = 1;
pub const LWS_CALLBACK_WS_SERVER_DROP_PROTOCOL: lws_callback_reasons = 78;
pub const LWS_CALLBACK_WS_SERVER_BIND_PROTOCOL: lws_callback_reasons = 77;
pub const LWS_CALLBACK_CONFIRM_EXTENSION_OKAY: lws_callback_reasons = 25;
pub const LWS_CALLBACK_FILTER_PROTOCOL_CONNECTION: lws_callback_reasons = 20;
pub const LWS_CALLBACK_WS_PEER_INITIATED_CLOSE: lws_callback_reasons = 38;
pub const LWS_CALLBACK_RECEIVE_PONG: lws_callback_reasons = 7;
pub const LWS_CALLBACK_RECEIVE: lws_callback_reasons = 6;
pub const LWS_CALLBACK_SERVER_WRITEABLE: lws_callback_reasons = 11;
pub const LWS_CALLBACK_CLOSED: lws_callback_reasons = 4;
pub const LWS_CALLBACK_ESTABLISHED: lws_callback_reasons = 0;
pub const LWS_CALLBACK_CLIENT_HTTP_DROP_PROTOCOL: lws_callback_reasons = 76;
pub const LWS_CALLBACK_CLIENT_HTTP_BIND_PROTOCOL: lws_callback_reasons = 85;
pub const LWS_CALLBACK_CLIENT_HTTP_WRITEABLE: lws_callback_reasons = 57;
pub const LWS_CALLBACK_COMPLETED_CLIENT_HTTP: lws_callback_reasons = 47;
pub const LWS_CALLBACK_RECEIVE_CLIENT_HTTP: lws_callback_reasons = 46;
pub const LWS_CALLBACK_RECEIVE_CLIENT_HTTP_READ: lws_callback_reasons = 48;
pub const LWS_CALLBACK_CLOSED_CLIENT_HTTP: lws_callback_reasons = 45;
pub const LWS_CALLBACK_ESTABLISHED_CLIENT_HTTP: lws_callback_reasons = 44;
pub const LWS_CALLBACK_HTTP_CONFIRM_UPGRADE: lws_callback_reasons = 86;
pub const LWS_CALLBACK_HTTP_DROP_PROTOCOL: lws_callback_reasons = 50;
pub const LWS_CALLBACK_HTTP_BIND_PROTOCOL: lws_callback_reasons = 49;
pub const LWS_CALLBACK_PROCESS_HTML: lws_callback_reasons = 52;
pub const LWS_CALLBACK_CHECK_ACCESS_RIGHTS: lws_callback_reasons = 51;
pub const LWS_CALLBACK_VERIFY_BASIC_AUTHORIZATION: lws_callback_reasons = 102;
pub const LWS_CALLBACK_ADD_HEADERS: lws_callback_reasons = 53;
pub const LWS_CALLBACK_FILTER_HTTP_CONNECTION: lws_callback_reasons = 18;
pub const LWS_CALLBACK_CLOSED_HTTP: lws_callback_reasons = 5;
pub const LWS_CALLBACK_HTTP_WRITEABLE: lws_callback_reasons = 16;
pub const LWS_CALLBACK_HTTP_FILE_COMPLETION: lws_callback_reasons = 15;
pub const LWS_CALLBACK_HTTP_BODY_COMPLETION: lws_callback_reasons = 14;
pub const LWS_CALLBACK_HTTP_BODY: lws_callback_reasons = 13;
pub const LWS_CALLBACK_HTTP: lws_callback_reasons = 12;
pub const LWS_CALLBACK_SERVER_NEW_CLIENT_INSTANTIATED: lws_callback_reasons =
    19;
pub const LWS_CALLBACK_OPENSSL_PERFORM_SERVER_CERT_VERIFICATION:
          lws_callback_reasons =
    58;
pub const LWS_CALLBACK_SSL_INFO: lws_callback_reasons = 67;
pub const LWS_CALLBACK_OPENSSL_CONTEXT_REQUIRES_PRIVATE_KEY:
          lws_callback_reasons =
    37;
pub const LWS_CALLBACK_OPENSSL_PERFORM_CLIENT_CERT_VERIFICATION:
          lws_callback_reasons =
    23;
pub const LWS_CALLBACK_OPENSSL_LOAD_EXTRA_SERVER_VERIFY_CERTS:
          lws_callback_reasons =
    22;
pub const LWS_CALLBACK_OPENSSL_LOAD_EXTRA_CLIENT_VERIFY_CERTS:
          lws_callback_reasons =
    21;
pub const LWS_CALLBACK_WSI_TX_CREDIT_GET: lws_callback_reasons = 103;
pub const LWS_CALLBACK_WSI_DESTROY: lws_callback_reasons = 30;
pub const LWS_CALLBACK_WSI_CREATE: lws_callback_reasons = 29;
pub const LWS_CALLBACK_PROTOCOL_DESTROY: lws_callback_reasons = 28;
pub const LWS_CALLBACK_PROTOCOL_INIT: lws_callback_reasons = 27;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_dll2 {
    pub prev: *mut lws_dll2,
    pub next: *mut lws_dll2,
    pub owner: *mut lws_dll2_owner,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_dll2_owner {
    pub tail: *mut lws_dll2,
    pub head: *mut lws_dll2,
    pub count: uint32_t,
}
pub type lws_dll2_t = lws_dll2;
pub type lws_dll2_owner_t = lws_dll2_owner;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_state_notify_link {
    pub list: lws_dll2_t,
    pub notify_cb: lws_state_notify_t,
    pub name: *const libc::c_char,
}
pub type lws_state_notify_t
    =
    Option<unsafe extern "C" fn(_: *mut lws_state_manager,
                                _: *mut lws_state_notify_link, _: libc::c_int,
                                _: libc::c_int) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_state_manager {
    pub notify_list: lws_dll2_owner_t,
    pub parent: *mut libc::c_void,
    pub state_names: *mut *const libc::c_char,
    pub name: *const libc::c_char,
    pub state: libc::c_int,
}
pub type lws_state_notify_link_t = lws_state_notify_link;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_retry_bo {
    pub retry_ms_table: *const uint32_t,
    pub retry_ms_table_count: uint16_t,
    pub conceal_count: uint16_t,
    pub secs_since_valid_ping: uint16_t,
    pub secs_since_valid_hangup: uint16_t,
    pub jitter_percent: uint8_t,
}
pub type lws_retry_bo_t = lws_retry_bo;
pub type lws_system_states_t = libc::c_uint;
pub const LWS_SYSTATE_POLICY_INVALID: lws_system_states_t = 11;
pub const LWS_SYSTATE_OPERATIONAL: lws_system_states_t = 10;
pub const LWS_SYSTATE_AUTH2: lws_system_states_t = 9;
pub const LWS_SYSTATE_AUTH1: lws_system_states_t = 8;
pub const LWS_SYSTATE_REGISTERED: lws_system_states_t = 7;
pub const LWS_SYSTATE_POLICY_VALID: lws_system_states_t = 6;
pub const LWS_SYSTATE_TIME_VALID: lws_system_states_t = 5;
pub const LWS_SYSTATE_DHCP: lws_system_states_t = 4;
pub const LWS_SYSTATE_IFACE_COLDPLUG: lws_system_states_t = 3;
pub const LWS_SYSTATE_INITIALIZED: lws_system_states_t = 2;
pub const LWS_SYSTATE_CONTEXT_CREATED: lws_system_states_t = 1;
pub const LWS_SYSTATE_UNKNOWN: lws_system_states_t = 0;
pub type lws_attach_cb_t
    =
    Option<unsafe extern "C" fn(_: *mut lws_context, _: libc::c_int,
                                _: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_system_ops {
    pub reboot: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub set_clock: Option<unsafe extern "C" fn(_: lws_usec_t) -> libc::c_int>,
    pub attach: Option<unsafe extern "C" fn(_: *mut lws_context,
                                            _: libc::c_int,
                                            _: lws_attach_cb_t,
                                            _: lws_system_states_t,
                                            _: *mut libc::c_void,
                                            _: *mut *mut lws_attach_item)
                           -> libc::c_int>,
}
pub type lws_system_ops_t = lws_system_ops;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_detlat {
    pub earliest_write_req: lws_usec_t,
    pub earliest_write_req_pre_write: lws_usec_t,
    pub aux: *const libc::c_char,
    pub type_0: libc::c_int,
    pub latencies: [uint32_t; 5],
    pub req_size: size_t,
    pub acc_size: size_t,
}
pub type lws_detlat_t = lws_detlat;
pub type det_lat_buf_cb_t
    =
    Option<unsafe extern "C" fn(_: *mut lws_context, _: *const lws_detlat_t)
               -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_protocol_vhost_options {
    pub next: *const lws_protocol_vhost_options,
    pub options: *const lws_protocol_vhost_options,
    pub name: *const libc::c_char,
    pub value: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_plat_file_ops {
    pub open: Option<unsafe extern "C" fn(_: *const lws_plat_file_ops,
                                          _: *const libc::c_char,
                                          _: *const libc::c_char,
                                          _: *mut lws_fop_flags_t)
                         -> lws_fop_fd_t>,
    pub close: Option<unsafe extern "C" fn(_: *mut lws_fop_fd_t)
                          -> libc::c_int>,
    pub seek_cur: Option<unsafe extern "C" fn(_: lws_fop_fd_t,
                                              _: lws_fileofs_t)
                             -> lws_fileofs_t>,
    pub read: Option<unsafe extern "C" fn(_: lws_fop_fd_t,
                                          _: *mut lws_filepos_t,
                                          _: *mut uint8_t, _: lws_filepos_t)
                         -> libc::c_int>,
    pub write: Option<unsafe extern "C" fn(_: lws_fop_fd_t,
                                           _: *mut lws_filepos_t,
                                           _: *mut uint8_t, _: lws_filepos_t)
                          -> libc::c_int>,
    pub fi: [lws_fops_index; 3],
    pub next: *const lws_plat_file_ops,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_fops_index {
    pub sig: *const libc::c_char,
    pub len: uint8_t,
}
pub type lws_fop_fd_t = *mut lws_fop_fd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_fop_fd {
    pub fd: lws_filefd_type,
    pub fops: *const lws_plat_file_ops,
    pub filesystem_priv: *mut libc::c_void,
    pub pos: lws_filepos_t,
    pub len: lws_filepos_t,
    pub flags: lws_fop_flags_t,
    pub mod_time: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_context_creation_info {
    pub port: libc::c_int,
    pub iface: *const libc::c_char,
    pub protocols: *const lws_protocols,
    pub extensions: *const lws_extension,
    pub token_limits: *const lws_token_limits,
    pub ssl_private_key_password: *const libc::c_char,
    pub ssl_cert_filepath: *const libc::c_char,
    pub ssl_private_key_filepath: *const libc::c_char,
    pub ssl_ca_filepath: *const libc::c_char,
    pub ssl_cipher_list: *const libc::c_char,
    pub http_proxy_address: *const libc::c_char,
    pub http_proxy_port: libc::c_uint,
    pub gid: libc::c_int,
    pub uid: libc::c_int,
    pub options: uint64_t,
    pub user: *mut libc::c_void,
    pub ka_time: libc::c_int,
    pub ka_probes: libc::c_int,
    pub ka_interval: libc::c_int,
    pub provided_client_ssl_ctx: *mut SSL_CTX,
    pub max_http_header_data: libc::c_ushort,
    pub max_http_header_pool: libc::c_ushort,
    pub count_threads: libc::c_uint,
    pub fd_limit_per_thread: libc::c_uint,
    pub timeout_secs: libc::c_uint,
    pub ecdh_curve: *const libc::c_char,
    pub vhost_name: *const libc::c_char,
    pub plugin_dirs: *const *const libc::c_char,
    pub pvo: *const lws_protocol_vhost_options,
    pub keepalive_timeout: libc::c_int,
    pub log_filepath: *const libc::c_char,
    pub mounts: *const lws_http_mount,
    pub server_string: *const libc::c_char,
    pub pt_serv_buf_size: libc::c_uint,
    pub max_http_header_data2: libc::c_uint,
    pub ssl_options_set: libc::c_long,
    pub ssl_options_clear: libc::c_long,
    pub ws_ping_pong_interval: libc::c_ushort,
    pub headers: *const lws_protocol_vhost_options,
    pub reject_service_keywords: *const lws_protocol_vhost_options,
    pub external_baggage_free_on_destroy: *mut libc::c_void,
    pub client_ssl_private_key_password: *const libc::c_char,
    pub client_ssl_cert_filepath: *const libc::c_char,
    pub client_ssl_cert_mem: *const libc::c_void,
    pub client_ssl_cert_mem_len: libc::c_uint,
    pub client_ssl_private_key_filepath: *const libc::c_char,
    pub client_ssl_ca_filepath: *const libc::c_char,
    pub client_ssl_ca_mem: *const libc::c_void,
    pub client_ssl_ca_mem_len: libc::c_uint,
    pub client_ssl_cipher_list: *const libc::c_char,
    pub fops: *const lws_plat_file_ops,
    pub simultaneous_ssl_restriction: libc::c_int,
    pub socks_proxy_address: *const libc::c_char,
    pub socks_proxy_port: libc::c_uint,
    pub bind_iface: libc::c_int,
    pub ssl_info_event_mask: libc::c_int,
    pub timeout_secs_ah_idle: libc::c_uint,
    pub ip_limit_ah: libc::c_ushort,
    pub ip_limit_wsi: libc::c_ushort,
    pub http2_settings: [uint32_t; 7],
    pub error_document_404: *const libc::c_char,
    pub alpn: *const libc::c_char,
    pub foreign_loops: *mut *mut libc::c_void,
    pub signal_cb: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                               _: libc::c_int) -> ()>,
    pub pcontext: *mut *mut lws_context,
    pub finalize: Option<unsafe extern "C" fn(_: *mut lws_vhost,
                                              _: *mut libc::c_void) -> ()>,
    pub finalize_arg: *mut libc::c_void,
    pub max_http_header_pool2: libc::c_uint,
    pub ssl_client_options_set: libc::c_long,
    pub ssl_client_options_clear: libc::c_long,
    pub tls1_3_plus_cipher_list: *const libc::c_char,
    pub client_tls_1_3_plus_cipher_list: *const libc::c_char,
    pub listen_accept_role: *const libc::c_char,
    pub listen_accept_protocol: *const libc::c_char,
    pub pprotocols: *mut *const lws_protocols,
    pub server_ssl_cert_mem: *const libc::c_void,
    pub server_ssl_cert_mem_len: libc::c_uint,
    pub server_ssl_private_key_mem: *const libc::c_void,
    pub server_ssl_private_key_mem_len: libc::c_uint,
    pub server_ssl_ca_mem: *const libc::c_void,
    pub server_ssl_ca_mem_len: libc::c_uint,
    pub username: *const libc::c_char,
    pub groupname: *const libc::c_char,
    pub unix_socket_perms: *const libc::c_char,
    pub system_ops: *const lws_system_ops_t,
    pub detailed_latency_cb: det_lat_buf_cb_t,
    pub detailed_latency_filepath: *const libc::c_char,
    pub retry_and_idle_policy: *const lws_retry_bo_t,
    pub register_notifier_list: *const *mut lws_state_notify_link_t,
    pub udp_loss_sim_tx_pc: uint8_t,
    pub udp_loss_sim_rx_pc: uint8_t,
    pub _unused: [*mut libc::c_void; 2],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct lws_http_mount {
    pub mount_next: *const lws_http_mount,
    pub mountpoint: *const libc::c_char,
    pub origin: *const libc::c_char,
    pub def: *const libc::c_char,
    pub protocol: *const libc::c_char,
    pub cgienv: *const lws_protocol_vhost_options,
    pub extra_mimetypes: *const lws_protocol_vhost_options,
    pub interpret: *const lws_protocol_vhost_options,
    pub cgi_timeout: libc::c_int,
    pub cache_max_age: libc::c_int,
    pub auth_mask: libc::c_uint,
    #[bitfield(name = "cache_reusable", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "cache_revalidate", ty = "libc::c_uint", bits =
               "1..=1")]
    #[bitfield(name = "cache_intermediaries", ty = "libc::c_uint", bits =
               "2..=2")]
    pub cache_reusable_cache_revalidate_cache_intermediaries: [u8; 1],
    pub origin_protocol: libc::c_uchar,
    pub mountpoint_len: libc::c_uchar,
    pub basic_auth_login_file: *const libc::c_char,
    pub _unused: [*mut libc::c_void; 2],
}
pub type lws_client_connect_ssl_connection_flags = libc::c_uint;
pub const LCCSCF_MUXABLE_STREAM: lws_client_connect_ssl_connection_flags =
    131072;
pub const LCCSCF_PIPELINE: lws_client_connect_ssl_connection_flags = 65536;
pub const LCCSCF_HTTP_NO_FOLLOW_REDIRECT:
          lws_client_connect_ssl_connection_flags =
    4096;
pub const LCCSCF_HTTP_X_WWW_FORM_URLENCODED:
          lws_client_connect_ssl_connection_flags =
    2048;
pub const LCCSCF_HTTP_MULTIPART_MIME: lws_client_connect_ssl_connection_flags
          =
    1024;
pub const LCCSCF_H2_MANUAL_RXFLOW: lws_client_connect_ssl_connection_flags =
    512;
pub const LCCSCF_H2_HEXIFY_AUTH_TOKEN: lws_client_connect_ssl_connection_flags
          =
    256;
pub const LCCSCF_H2_AUTH_BEARER: lws_client_connect_ssl_connection_flags =
    128;
pub const LCCSCF_H2_QUIRK_OVERFLOWS_TXCR:
          lws_client_connect_ssl_connection_flags =
    64;
pub const LCCSCF_H2_QUIRK_NGHTTP2_END_STREAM:
          lws_client_connect_ssl_connection_flags =
    32;
pub const LCCSCF_ALLOW_INSECURE: lws_client_connect_ssl_connection_flags = 16;
pub const LCCSCF_ALLOW_EXPIRED: lws_client_connect_ssl_connection_flags = 8;
pub const LCCSCF_SKIP_SERVER_CERT_HOSTNAME_CHECK:
          lws_client_connect_ssl_connection_flags =
    4;
pub const LCCSCF_ALLOW_SELFSIGNED: lws_client_connect_ssl_connection_flags =
    2;
pub const LCCSCF_USE_SSL: lws_client_connect_ssl_connection_flags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lws_client_connect_info {
    pub context: *mut lws_context,
    pub address: *const libc::c_char,
    pub port: libc::c_int,
    pub ssl_connection: libc::c_int,
    pub path: *const libc::c_char,
    pub host: *const libc::c_char,
    pub origin: *const libc::c_char,
    pub protocol: *const libc::c_char,
    pub ietf_version_or_minus_one: libc::c_int,
    pub userdata: *mut libc::c_void,
    pub client_exts: *const libc::c_void,
    pub method: *const libc::c_char,
    pub parent_wsi: *mut lws,
    pub uri_replace_from: *const libc::c_char,
    pub uri_replace_to: *const libc::c_char,
    pub vhost: *mut lws_vhost,
    pub pwsi: *mut *mut lws,
    pub iface: *const libc::c_char,
    pub local_protocol_name: *const libc::c_char,
    pub alpn: *const libc::c_char,
    pub seq: *mut lws_sequencer,
    pub opaque_user_data: *mut libc::c_void,
    pub retry_and_idle_policy: *const lws_retry_bo_t,
    pub manual_initial_tx_credit: libc::c_int,
    pub sys_tls_client_cert: uint8_t,
    pub mqtt_cp: *mut libc::c_void,
    pub _unused: [*mut libc::c_void; 4],
}
pub type lws_write_protocol = libc::c_uint;
pub const LWS_WRITE_CLIENT_IGNORE_XOR_MASK: lws_write_protocol = 128;
pub const LWS_WRITE_H2_STREAM_END: lws_write_protocol = 128;
pub const LWS_WRITE_NO_FIN: lws_write_protocol = 64;
pub const LWS_WRITE_BUFLIST: lws_write_protocol = 32;
pub const LWS_WRITE_HTTP_HEADERS_CONTINUATION: lws_write_protocol = 9;
pub const LWS_WRITE_HTTP_HEADERS: lws_write_protocol = 8;
pub const LWS_WRITE_HTTP_FINAL: lws_write_protocol = 7;
pub const LWS_WRITE_PONG: lws_write_protocol = 6;
pub const LWS_WRITE_PING: lws_write_protocol = 5;
pub const LWS_WRITE_HTTP: lws_write_protocol = 3;
pub const LWS_WRITE_CONTINUATION: lws_write_protocol = 2;
pub const LWS_WRITE_BINARY: lws_write_protocol = 1;
pub const LWS_WRITE_TEXT: lws_write_protocol = 0;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type __free_fn_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_buff {
    pub buff: *mut libc::c_char,
    pub start: *mut libc::c_char,
    pub left: libc::c_int,
    pub next: *mut t_buff,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_rc_room {
    pub rid: *mut libc::c_char,
    pub fname: *mut libc::c_char,
    pub name: *mut libc::c_char,
    pub topic: *mut libc::c_char,
    pub ls: libc::c_ulonglong,
    pub t: libc::c_char,
    pub next: *mut t_rc_room,
}
pub type t_rc_id = [libc::c_char; 16];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_sess {
    pub irc_fd: libc::c_int,
    pub rch_fd: libc::c_int,
    pub state: libc::c_int,
    pub irc: C2RustUnnamed_2,
    pub irc_buff: [libc::c_char; 513],
    pub irc_buff_head: libc::c_int,
    pub poll: *mut pollfd,
    pub rc_poll: *mut pollfd,
    pub irc_out_buff: *mut t_buff,
    pub irc_out_buff_tail: *mut *mut t_buff,
    pub rc: C2RustUnnamed_1,
    pub next: *mut t_sess,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub context: *mut lws_context,
    pub vhost: *mut lws_vhost,
    pub protocol: *const lws_protocols,
    pub pthread_spam: [pthread_t; 2],
    pub out_buff: *mut t_buff,
    pub out_buff_tail: *mut *mut t_buff,
    pub tail: uint32_t,
    pub i: lws_client_connect_info,
    pub client_wsi: *mut lws,
    pub counter: libc::c_int,
    pub fd_idx: libc::c_int,
    pub finished: libc::c_char,
    pub established: libc::c_char,
    pub last_ping: time_t,
    pub self_id: *mut libc::c_char,
    pub token: *mut libc::c_char,
    pub commands: *mut t_rc_command,
    pub rooms: *mut t_rc_room,
    pub messages: *mut t_rc_message,
    pub in_messages: *mut t_rc_message,
    pub in_messages_tree: *mut libc::c_void,
    pub sent: *mut t_rc_sndmsg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_rc_sndmsg {
    pub next: *mut t_rc_sndmsg,
    pub id: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_rc_message {
    pub id: *const libc::c_char,
    pub t: libc::c_char,
    pub dstname: *mut libc::c_char,
    pub msg: *mut libc::c_char,
    pub sender: *mut libc::c_char,
    pub reactions: *mut libc::c_char,
    pub tim: time_t,
    pub next: *mut t_rc_message,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_rc_command {
    pub id: t_rc_id,
    pub data: *mut libc::c_void,
    pub fce: Option<unsafe extern "C" fn(_: *mut t_sess, _: *mut libc::c_void,
                                         _: *mut json_object) -> libc::c_int>,
    pub next: *mut t_rc_command,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub nick: *mut libc::c_char,
}
static mut MSG_edited: *mut libc::c_char =
    b"[edited]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut MSG_removed: *mut libc::c_char =
    b"[removed]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut server_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut interrupted: libc::c_int = 0;
#[no_mangle]
pub static mut pollfds: [pollfd; 512] =
    [pollfd{fd: 0, events: 0, revents: 0,}; 512];
#[no_mangle]
pub static mut sessions: [*mut t_sess; 512] =
    [0 as *const t_sess as *mut t_sess; 512];
#[no_mangle]
pub static mut maxfds: libc::c_int = 0 as libc::c_int;
static mut rc_timeout: libc::c_int = 60 as libc::c_int * 5 as libc::c_int;
#[no_mangle]
pub static mut selfident: *const libc::c_char =
    b"myserver\x00" as *const u8 as *const libc::c_char;
static mut session_list: *mut t_sess = 0 as *const t_sess as *mut t_sess;
#[no_mangle]
pub unsafe extern "C" fn poll_removefd(mut i: libc::c_int) {
    let mut s: *mut t_sess = session_list;
    while !s.is_null() {
        if (*s).rc_poll >=
               &mut *pollfds.as_mut_ptr().offset(i as isize) as *mut pollfd {
            (*s).rc_poll = (*s).rc_poll.offset(-1)
        }
        if (*s).poll >=
               &mut *pollfds.as_mut_ptr().offset(i as isize) as *mut pollfd {
            (*s).poll = (*s).poll.offset(-1)
        }
        s = (*s).next
    }
    memcpy(&mut *pollfds.as_mut_ptr().offset(i as isize) as *mut pollfd as
               *mut libc::c_void,
           &mut *pollfds.as_mut_ptr().offset((i + 1 as libc::c_int) as isize)
               as *mut pollfd as *const libc::c_void,
           (::std::mem::size_of::<pollfd>() as
                libc::c_ulong).wrapping_mul((maxfds - i) as libc::c_ulong));
    memcpy(&mut *sessions.as_mut_ptr().offset(i as isize) as *mut *mut t_sess
               as *mut libc::c_void,
           &mut *sessions.as_mut_ptr().offset((i + 1 as libc::c_int) as isize)
               as *mut *mut t_sess as *const libc::c_void,
           (::std::mem::size_of::<*mut t_sess>() as
                libc::c_ulong).wrapping_mul((maxfds - i) as libc::c_ulong));
    maxfds -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn poll_addfd(mut fd: libc::c_int, mut s: *mut t_sess,
                                    mut events: libc::c_int) -> libc::c_int {
    pollfds[maxfds as usize].fd = fd;
    pollfds[maxfds as usize].events = events as libc::c_short;
    sessions[maxfds as usize] = s;
    (*s).rc.fd_idx = maxfds;
    let fresh0 = maxfds;
    maxfds = maxfds + 1;
    return fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn sess__add_rc_out(mut s: *mut t_sess,
                                          mut len: libc::c_int,
                                          mut c: *const libc::c_char)
 -> libc::c_int {
    if len == -(1 as libc::c_int) { len = strlen(c) as libc::c_int }
    let mut buff: *mut t_buff = buff__new();
    (*buff).buff =
        malloc(((if (4 as libc::c_int + 10 as libc::c_int + 2 as libc::c_int)
                        % 16 as libc::c_int != 0 {
                     (4 as libc::c_int + 10 as libc::c_int + 2 as libc::c_int)
                         +
                         (16 as libc::c_int -
                              (4 as libc::c_int + 10 as libc::c_int +
                                   2 as libc::c_int) % 16 as libc::c_int)
                 } else {
                     (4 as libc::c_int + 10 as libc::c_int) + 2 as libc::c_int
                 }) + len + 1 as libc::c_int) as libc::c_ulong) as
            *mut libc::c_char;
    memcpy((*buff).buff.offset((if (4 as libc::c_int + 10 as libc::c_int +
                                        2 as libc::c_int) % 16 as libc::c_int
                                       != 0 {
                                    (4 as libc::c_int + 10 as libc::c_int +
                                         2 as libc::c_int) +
                                        (16 as libc::c_int -
                                             (4 as libc::c_int +
                                                  10 as libc::c_int +
                                                  2 as libc::c_int) %
                                                 16 as libc::c_int)
                                } else {
                                    (4 as libc::c_int + 10 as libc::c_int) +
                                        2 as libc::c_int
                                }) as isize) as *mut libc::c_void,
           c as *const libc::c_void, len as libc::c_ulong);
    *(*buff).buff.offset(((if (4 as libc::c_int + 10 as libc::c_int +
                                   2 as libc::c_int) % 16 as libc::c_int != 0
                              {
                               (4 as libc::c_int + 10 as libc::c_int +
                                    2 as libc::c_int) +
                                   (16 as libc::c_int -
                                        (4 as libc::c_int + 10 as libc::c_int
                                             + 2 as libc::c_int) %
                                            16 as libc::c_int)
                           } else {
                               (4 as libc::c_int + 10 as libc::c_int) +
                                   2 as libc::c_int
                           }) + len) as isize) =
        '\u{0}' as i32 as libc::c_char;
    (*buff).left = len;
    *(*s).rc.out_buff_tail = buff;
    (*s).rc.out_buff_tail = &mut (*buff).next;
    if !(*s).rc.client_wsi.is_null() {
        lws_callback_on_writable((*s).rc.client_wsi);
        (*(*s).rc_poll).events =
            ((*(*s).rc_poll).events as libc::c_int | 0x4 as libc::c_int) as
                libc::c_short
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__add_rc_out_(mut s: *mut t_sess,
                                           mut c: *const libc::c_char)
 -> libc::c_int {
    let mut r: libc::c_int = sess__add_rc_out(s, -(1 as libc::c_int), c);
    free(c as *mut libc::c_char as *mut libc::c_void);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_command_by_id(mut s: *mut t_sess,
                                                mut id: *const libc::c_char)
 -> *mut t_rc_command {
    let mut cmd: *mut t_rc_command = 0 as *mut t_rc_command;
    cmd = (*s).rc.commands;
    while !cmd.is_null() && strcmp((*cmd).id.as_mut_ptr(), id) != 0 {
        cmd = (*cmd).next
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_work(mut s: *mut t_sess,
                                       mut in_0: *const libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut collection: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut selfuser: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut userid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut _id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut username: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut roomName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args: *mut json_object = 0 as *mut json_object;
    let mut jo: *mut json_object = 0 as *mut json_object;
    let mut jobj: *mut json_object = json_tokener_parse(in_0);
    if jobj.is_null() {
        logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
             b"Can\'t json_tokener_parse()\n\x00" as *const u8 as
                 *const libc::c_char);
        return -(1 as libc::c_int)
    }
    jo =
        json_object_object_get(jobj,
                               b"msg\x00" as *const u8 as
                                   *const libc::c_char);
    if !jo.is_null() {
        let mut msg_0: *const libc::c_char = json_object_get_string(jo);
        if strcmp(msg_0, b"ping\x00" as *const u8 as *const libc::c_char) == 0
           {
            msg_0 = 0 as *const libc::c_char;
            (*s).rc.last_ping = time(0 as *mut time_t);
            sess__add_rc_out(s, -(1 as libc::c_int),
                             b"{\"msg\":\"pong\"}\x00" as *const u8 as
                                 *const libc::c_char);
            current_block = 5470224165756977677;
        } else {
            msg_0 = 0 as *const libc::c_char;
            current_block = 1856101646708284338;
        }
    } else { current_block = 1856101646708284338; }
    match current_block {
        1856101646708284338 => {
            jo =
                json_object_object_get(jobj,
                                       b"id\x00" as *const u8 as
                                           *const libc::c_char);
            if !jo.is_null() {
                let mut id: *const libc::c_char = json_object_get_string(jo);
                let mut cb: *mut t_rc_command = sess__rc_command_by_id(s, id);
                if !cb.is_null() {
                    logg(((1 as libc::c_int) << 1 as libc::c_int) as
                             libc::c_short,
                         b"I\'ve GOT REPLY %s!\n\x00" as *const u8 as
                             *const libc::c_char, id);
                    if (*cb).fce.is_some() {
                        r =
                            (*cb).fce.expect("non-null function pointer")(s,
                                                                          (*cb).data,
                                                                          jobj)
                    }
                    current_block = 5470224165756977677;
                } else { current_block = 11584701595673473500; }
            } else { current_block = 11584701595673473500; }
            match current_block {
                5470224165756977677 => { }
                _ => {
                    if json_read(0 as *mut libc::c_void, jobj,
                                 b"{msg=%s collection=%s id:%s fields:{username:%s}}\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"added\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"users\x00" as *const u8 as
                                     *const libc::c_char,
                                 &mut userid as *mut *mut libc::c_char,
                                 &mut selfuser as *mut *mut libc::c_char) ==
                           2 as libc::c_int {
                        sess__irc_nick_set(s, selfuser);
                        (*s).rc.self_id = userid;
                        userid = 0 as *mut libc::c_char;
                        let mut allsubs: [*mut libc::c_char; 5] =
                            [b"notification\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             b"rooms-changed\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             b"subscriptions-changed\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             b"otr\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char, 0 as *mut libc::c_char];
                        let mut subs: *mut *mut libc::c_char =
                            allsubs.as_mut_ptr();
                        while !(*subs).is_null() {
                            let mut su: [libc::c_char; 64] = [0; 64];
                            snprintf(su.as_mut_ptr(),
                                     ::std::mem::size_of::<[libc::c_char; 64]>()
                                         as libc::c_ulong,
                                     b"%s/%s\x00" as *const u8 as
                                         *const libc::c_char, (*s).rc.self_id,
                                     *subs);
                            sess__rc_command_call_(s, 0 as *mut libc::c_void,
                                                   json_create(0 as
                                                                   *mut libc::c_void,
                                                               b"{msg:%s name:%s params:[%s %b]}\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"sub\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"stream-notify-user\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               su.as_mut_ptr(),
                                                               0 as
                                                                   libc::c_int),
                                                   None);
                            subs = subs.offset(1)
                        }
                    } else if json_read(0 as *mut libc::c_void, jobj,
                                        b"{collection=%s msg=%s fields:{args:[:{t=%s u:{username:%s}} :{roomName:%s}]}}\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        b"stream-room-messages\x00" as
                                            *const u8 as *const libc::c_char,
                                        b"changed\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"uj\x00" as *const u8 as
                                            *const libc::c_char,
                                        &mut username as
                                            *mut *mut libc::c_char,
                                        &mut roomName as
                                            *mut *mut libc::c_char) ==
                                  2 as libc::c_int {
                        sess__add_irc_out(s,
                                          buff__sprintf(b":%s JOIN #%s\r\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        username, roomName));
                    } else if json_read(0 as *mut libc::c_void, jobj,
                                        b"{msg:%s collection:%s fields:{args:%o}}\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        &mut msg as *mut *mut libc::c_char,
                                        &mut collection as
                                            *mut *mut libc::c_char,
                                        &mut args as *mut *mut json_object) ==
                                  3 as libc::c_int {
                        if strcmp(collection,
                                  b"stream-notify-user\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            let mut name: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut fname: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut rid: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut t: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            if json_read(0 as *mut libc::c_void, args,
                                         b"[=%s :{name:%s fname:%s rid:%s t:%s}]\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"inserted\x00" as *const u8 as
                                             *const libc::c_char,
                                         &mut name as *mut *mut libc::c_char,
                                         &mut fname as *mut *mut libc::c_char,
                                         &mut rid as *mut *mut libc::c_char,
                                         &mut t as *mut *mut libc::c_char) ==
                                   4 as libc::c_int {
                                sess__rc_room_add(s,
                                                  *t.offset(0 as libc::c_int
                                                                as isize),
                                                  name, rid, fname,
                                                  0 as *const libc::c_char);
                                if !fname.is_null() {
                                    free(fname as *mut libc::c_void);
                                    fname = 0 as *mut libc::c_char
                                }
                                if !name.is_null() {
                                    free(name as *mut libc::c_void);
                                    name = 0 as *mut libc::c_char
                                }
                                if !rid.is_null() {
                                    free(rid as *mut libc::c_void);
                                    rid = 0 as *mut libc::c_char
                                }
                                if !t.is_null() {
                                    free(t as *mut libc::c_void);
                                    t = 0 as *mut libc::c_char
                                }
                                sess__rc_queue_process(s);
                            }
                        } else {
                            let mut cnt_args: libc::c_int =
                                json_object_array_length(args) as libc::c_int;
                            let mut i: libc::c_int = 0 as libc::c_int;
                            while i < cnt_args {
                                let mut attachments: *mut json_object =
                                    0 as *mut json_object;
                                let mut reactions: *mut json_object =
                                    0 as *mut json_object;
                                let mut p: *mut json_object =
                                    json_object_array_get_idx(args,
                                                              i as size_t);
                                let mut msg_1: *mut libc::c_char =
                                    0 as *mut libc::c_char;
                                let mut rid_0: *mut libc::c_char =
                                    0 as *mut libc::c_char;
                                let mut username_0: *mut libc::c_char =
                                    0 as *mut libc::c_char;
                                let mut roomName_0: *mut libc::c_char =
                                    0 as *mut libc::c_char;
                                let mut t_0: *mut libc::c_char =
                                    0 as *mut libc::c_char;
                                let mut tmid: *mut libc::c_char =
                                    0 as *mut libc::c_char;
                                let mut mt: *mut libc::c_char =
                                    0 as *mut libc::c_char;
                                let mut roomParticipant: libc::c_int =
                                    0 as libc::c_int;
                                r =
                                    json_read(0 as *mut libc::c_void, p,
                                              b"{payload:{message:{msg:%s} rid:%s sender:{username:%s} type:%s _id:%s}}\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              &mut msg_1 as
                                                  *mut *mut libc::c_char,
                                              &mut rid_0 as
                                                  *mut *mut libc::c_char,
                                              &mut username_0 as
                                                  *mut *mut libc::c_char,
                                              &mut t_0 as
                                                  *mut *mut libc::c_char,
                                              &mut _id as
                                                  *mut *mut libc::c_char);
                                if r >= 5 as libc::c_int {
                                    let mut dst: *mut libc::c_char =
                                        (*s).irc.nick;
                                    if strcmp(dst, username_0) == 0 {
                                        let mut r_0: *mut t_rc_room =
                                            sess__rc_room_by_rid(s, rid_0);
                                        if !r_0.is_null() {
                                            dst = (*r_0).name
                                        }
                                    }
                                    if !(sess__find_sent_message(s, _id) != 0)
                                       {
                                        sess__irc_send_message(s,
                                                               'd' as i32 as
                                                                   libc::c_char,
                                                               username_0,
                                                               dst,
                                                               0 as
                                                                   *mut libc::c_char,
                                                               msg_1);
                                    }
                                } else {
                                    r =
                                        json_read(0 as *mut libc::c_void, p,
                                                  b"{msg:%s rid:%s _id:%s u:{username:%s} attachments?%o reactions?%o tmid?%s t?%s}\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  &mut msg_1 as
                                                      *mut *mut libc::c_char,
                                                  &mut rid_0 as
                                                      *mut *mut libc::c_char,
                                                  &mut _id as
                                                      *mut *mut libc::c_char,
                                                  &mut username_0 as
                                                      *mut *mut libc::c_char,
                                                  &mut attachments as
                                                      *mut *mut json_object,
                                                  &mut reactions as
                                                      *mut *mut json_object,
                                                  &mut tmid as
                                                      *mut *mut libc::c_char,
                                                  &mut mt as
                                                      *mut *mut libc::c_char);
                                    if r >= 4 as libc::c_int &&
                                           !msg_1.is_null() &&
                                           !rid_0.is_null() && !_id.is_null()
                                           && !username_0.is_null() {
                                        let mut room: *mut t_rc_room =
                                            sess__rc_room_by_rid(s, rid_0);
                                        if (i + 1 as libc::c_int) < cnt_args {
                                            i += 1 as libc::c_int;
                                            let mut p_0: *mut json_object =
                                                json_object_array_get_idx(args,
                                                                          i as
                                                                              size_t);
                                            r =
                                                json_read(0 as
                                                              *mut libc::c_void,
                                                          p_0,
                                                          b"{roomType:%s roomName?%s roomParticipant?%b}\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          &mut t_0 as
                                                              *mut *mut libc::c_char,
                                                          &mut roomName_0 as
                                                              *mut *mut libc::c_char,
                                                          &mut roomParticipant
                                                              as
                                                              *mut libc::c_int)
                                        }
                                        if !(sess__find_sent_message(s, _id)
                                                 != 0) {
                                            let mut s_reactions:
                                                    *mut libc::c_char =
                                                reactions2string(reactions);
                                            let mut _pfx: *mut libc::c_char =
                                                0 as *mut libc::c_char;
                                            let mut _msg: *mut libc::c_char =
                                                msg_1;
                                            let mut m: *mut t_rc_message =
                                                sess__rc_find_message(s, _id);
                                            if !m.is_null() {
                                                logg(((1 as libc::c_int) <<
                                                          4 as libc::c_int) as
                                                         libc::c_short,
                                                     b"Repeated message %s\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     _id);
                                                if !s_reactions.is_null() &&
                                                       ((*m).reactions.is_null()
                                                            ||
                                                            strcmp((*m).reactions,
                                                                   s_reactions)
                                                                != 0) {
                                                    let mut __c:
                                                            *mut libc::c_char =
                                                        0 as
                                                            *mut libc::c_char;
                                                    __c = (*m).reactions;
                                                    (*m).reactions =
                                                        s_reactions;
                                                    s_reactions = __c;
                                                    _pfx = (*m).reactions
                                                }
                                                if strcmp((*m).msg, msg_1) !=
                                                       0 {
                                                    let mut __c_0:
                                                            *mut libc::c_char =
                                                        0 as
                                                            *mut libc::c_char;
                                                    __c_0 = (*m).msg;
                                                    (*m).msg = msg_1;
                                                    msg_1 = __c_0;
                                                    _pfx = MSG_edited;
                                                    if !mt.is_null() &&
                                                           strcmp(mt,
                                                                  b"rm\x00" as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                                               == 0 {
                                                        _pfx = MSG_removed;
                                                        _msg = msg_1
                                                    }
                                                }
                                                if _pfx.is_null() {
                                                    current_block =
                                                        1799707834802464914;
                                                } else {
                                                    current_block =
                                                        2793352396589381719;
                                                }
                                            } else {
                                                sess__rc_add_message(s, _id,
                                                                     msg_1,
                                                                     username_0,
                                                                     s_reactions);
                                                current_block =
                                                    2793352396589381719;
                                            }
                                            match current_block {
                                                1799707834802464914 => { }
                                                _ => {
                                                    let mut _dest:
                                                            *mut libc::c_char =
                                                        roomName_0;
                                                    let mut _t: libc::c_char =
                                                        'd' as i32 as
                                                            libc::c_char;
                                                    if !t_0.is_null() &&
                                                           (*t_0 as
                                                                libc::c_int ==
                                                                'c' as i32 ||
                                                                *t_0 as
                                                                    libc::c_int
                                                                    ==
                                                                    'p' as
                                                                        i32)
                                                           &&
                                                           !roomName_0.is_null()
                                                       {
                                                        _t =
                                                            'c' as i32 as
                                                                libc::c_char
                                                    } else if !room.is_null()
                                                                  &&
                                                                  strcmp(username_0,
                                                                         (*s).irc.nick)
                                                                      == 0 {
                                                        _dest = (*room).name
                                                    } else {
                                                        _dest = (*s).irc.nick
                                                    }
                                                    sess__irc_send_message(s,
                                                                           _t,
                                                                           username_0,
                                                                           _dest,
                                                                           _pfx,
                                                                           _msg);
                                                }
                                            }
                                        }
                                    }
                                }
                                if !msg_1.is_null() {
                                    free(msg_1 as *mut libc::c_void);
                                    msg_1 = 0 as *mut libc::c_char
                                }
                                if !rid_0.is_null() {
                                    free(rid_0 as *mut libc::c_void);
                                    rid_0 = 0 as *mut libc::c_char
                                }
                                if !username_0.is_null() {
                                    free(username_0 as *mut libc::c_void);
                                    username_0 = 0 as *mut libc::c_char
                                }
                                if !roomName_0.is_null() {
                                    free(roomName_0 as *mut libc::c_void);
                                    roomName_0 = 0 as *mut libc::c_char
                                }
                                if !t_0.is_null() {
                                    free(t_0 as *mut libc::c_void);
                                    t_0 = 0 as *mut libc::c_char
                                }
                                if !mt.is_null() {
                                    free(mt as *mut libc::c_void);
                                    mt = 0 as *mut libc::c_char
                                }
                                if !tmid.is_null() {
                                    free(tmid as *mut libc::c_void);
                                    tmid = 0 as *mut libc::c_char
                                }
                                i += 1
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    json_object_put(jobj);
    if !msg.is_null() {
        free(msg as *mut libc::c_void);
        msg = 0 as *mut libc::c_char
    }
    if !_id.is_null() {
        free(_id as *mut libc::c_void);
        _id = 0 as *mut libc::c_char
    }
    if !collection.is_null() {
        free(collection as *mut libc::c_void);
        collection = 0 as *mut libc::c_char
    }
    if !selfuser.is_null() {
        free(selfuser as *mut libc::c_void);
        selfuser = 0 as *mut libc::c_char
    }
    if !userid.is_null() {
        free(userid as *mut libc::c_void);
        userid = 0 as *mut libc::c_char
    }
    if !username.is_null() {
        free(username as *mut libc::c_void);
        username = 0 as *mut libc::c_char
    }
    if !roomName.is_null() {
        free(roomName as *mut libc::c_void);
        roomName = 0 as *mut libc::c_char
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn callback_minimal_broker_2(mut wsi: *mut lws,
                                               mut reason:
                                                   lws_callback_reasons,
                                               mut user: *mut libc::c_void,
                                               mut in_0: *mut libc::c_void,
                                               mut len: size_t)
 -> libc::c_int {
    let mut m: libc::c_int = 0;
    let mut s: *mut t_sess = lws_get_opaque_user_data(wsi) as *mut t_sess;
    if s.is_null() {
        return lws_callback_http_dummy(wsi, reason, user, in_0, len)
    }
    if (*s).rc.finished != 0 { return -(1 as libc::c_int) }
    let mut b: *mut t_buff = 0 as *mut t_buff;
    match reason as libc::c_uint {
        1 => {
            logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
                 b"CLIENT_CONNECTION_ERROR: %s\n\x00" as *const u8 as
                     *const libc::c_char,
                 if !in_0.is_null() {
                     in_0 as *mut libc::c_char
                 } else {
                     b"(null)\x00" as *const u8 as *const libc::c_char
                 });
            (*s).rc.client_wsi = 0 as *mut lws;
            (*s).rc.finished = 1 as libc::c_int as libc::c_char;
            sess__add_irc_out(s,
                              buff__sprintf(b"RocketChat connection error\r\n\x00"
                                                as *const u8 as
                                                *const libc::c_char));
            (*s).state |= 1 as libc::c_int;
            if !(*s).rc.vhost.is_null() && !(*s).rc.protocol.is_null() {
                lws_timed_callback_vh_protocol((*s).rc.vhost,
                                               (*s).rc.protocol,
                                               LWS_CALLBACK_USER as
                                                   libc::c_int,
                                               1 as libc::c_int);
            }
        }
        3 => {
            logg(((1 as libc::c_int) << 1 as libc::c_int) as libc::c_short,
                 b"%s: established\n\x00" as *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 26],
                                           &[libc::c_char; 26]>(b"callback_minimal_broker_2\x00")).as_ptr());
            (*s).rc.established = 1 as libc::c_int as libc::c_char
        }
        10 => {
            logg(((1 as libc::c_int) << 4 as libc::c_int) as libc::c_short,
                 b"LWS_CALLBACK_CLIENT_WRITEABLE\n\x00" as *const u8 as
                     *const libc::c_char);
            b = (*s).rc.out_buff;
            if b.is_null() {
                (*(*s).rc_poll).events = 0x1 as libc::c_int as libc::c_short
            } else {
                logg(((1 as libc::c_int) << 4 as libc::c_int) as
                         libc::c_short,
                     b"RC-TX[%d]: %.*s\n\x00" as *const u8 as
                         *const libc::c_char, (*(*s).rc_poll).fd, (*b).left,
                     ((*b).buff as
                          *const libc::c_char).offset((if (4 as libc::c_int +
                                                               10 as
                                                                   libc::c_int
                                                               +
                                                               2 as
                                                                   libc::c_int)
                                                              %
                                                              16 as
                                                                  libc::c_int
                                                              != 0 {
                                                           (4 as libc::c_int +
                                                                10 as
                                                                    libc::c_int
                                                                +
                                                                2 as
                                                                    libc::c_int)
                                                               +
                                                               (16 as
                                                                    libc::c_int
                                                                    -
                                                                    (4 as
                                                                         libc::c_int
                                                                         +
                                                                         10 as
                                                                             libc::c_int
                                                                         +
                                                                         2 as
                                                                             libc::c_int)
                                                                        %
                                                                        16 as
                                                                            libc::c_int)
                                                       } else {
                                                           (4 as libc::c_int +
                                                                10 as
                                                                    libc::c_int)
                                                               +
                                                               2 as
                                                                   libc::c_int
                                                       }) as isize));
                m =
                    lws_write(wsi,
                              ((*b).buff as
                                   *mut libc::c_uchar).offset((if (4 as
                                                                       libc::c_int
                                                                       +
                                                                       10 as
                                                                           libc::c_int
                                                                       +
                                                                       2 as
                                                                           libc::c_int)
                                                                      %
                                                                      16 as
                                                                          libc::c_int
                                                                      != 0 {
                                                                   (4 as
                                                                        libc::c_int
                                                                        +
                                                                        10 as
                                                                            libc::c_int
                                                                        +
                                                                        2 as
                                                                            libc::c_int)
                                                                       +
                                                                       (16 as
                                                                            libc::c_int
                                                                            -
                                                                            (4
                                                                                 as
                                                                                 libc::c_int
                                                                                 +
                                                                                 10
                                                                                     as
                                                                                     libc::c_int
                                                                                 +
                                                                                 2
                                                                                     as
                                                                                     libc::c_int)
                                                                                %
                                                                                16
                                                                                    as
                                                                                    libc::c_int)
                                                               } else {
                                                                   (4 as
                                                                        libc::c_int
                                                                        +
                                                                        10 as
                                                                            libc::c_int)
                                                                       +
                                                                       2 as
                                                                           libc::c_int
                                                               }) as isize),
                              (*b).left as size_t, LWS_WRITE_TEXT);
                if m < (*b).left {
                    logg(((1 as libc::c_int) << 0 as libc::c_int) as
                             libc::c_short,
                         b"ERROR %d writing to ws socket\n\x00" as *const u8
                             as *const libc::c_char, m);
                    return -(1 as libc::c_int)
                }
                if (*s).rc.out_buff_tail == &mut (*b).next as *mut *mut t_buff
                   {
                    (*s).rc.out_buff_tail = &mut (*s).rc.out_buff;
                    (*(*s).rc_poll).events =
                        0x1 as libc::c_int as libc::c_short
                } else { lws_callback_on_writable(wsi); }
                (*s).rc.out_buff = (*b).next;
                buff__free(b);
            }
        }
        75 => {
            logg(((1 as libc::c_int) << 1 as libc::c_int) as libc::c_short,
                 b"%s: LWS_CALLBACK_CLIENT_CLOSED\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 26],
                                           &[libc::c_char; 26]>(b"callback_minimal_broker_2\x00")).as_ptr());
            (*s).rc.client_wsi = 0 as *mut lws;
            (*s).rc.established = 0 as libc::c_int as libc::c_char;
            (*s).rc.finished = 1 as libc::c_int as libc::c_char;
            sess__add_irc_out(s,
                              buff__sprintf(b"RocketChat closing socket\r\n\x00"
                                                as *const u8 as
                                                *const libc::c_char));
            (*s).state |= 1 as libc::c_int;
            lws_timed_callback_vh_protocol((*s).rc.vhost, (*s).rc.protocol,
                                           LWS_CALLBACK_USER as libc::c_int,
                                           1 as libc::c_int);
        }
        71 => {
            if !(*s).rc.client_wsi.is_null() &&
                   (*s).rc.established as libc::c_int != 0 {
                lws_callback_on_writable((*s).rc.client_wsi);
            }
        }
        1000 => {
            logg(((1 as libc::c_int) << 4 as libc::c_int) as libc::c_short,
                 b"%s: LWS_CALLBACK_USER\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 26],
                                           &[libc::c_char; 26]>(b"callback_minimal_broker_2\x00")).as_ptr());
            lws_timed_callback_vh_protocol((*s).rc.vhost, (*s).rc.protocol,
                                           LWS_CALLBACK_USER as libc::c_int,
                                           1 as libc::c_int);
        }
        8 => {
            logg(((1 as libc::c_int) << 4 as libc::c_int) as libc::c_short,
                 b"RC-RX[%d]: %s\n\x00" as *const u8 as *const libc::c_char,
                 (*(*s).rc_poll).fd, in_0 as *const libc::c_char);
            sess__rc_work(s, in_0 as *const libc::c_char);
        }
        _ => { }
    }
    return lws_callback_http_dummy(wsi, reason, user, in_0, len);
}
static mut protocols: [lws_protocols; 2] =
    unsafe {
        [{
             let mut init =
                 lws_protocols{name:
                                   b"lws-minimal-broker\x00" as *const u8 as
                                       *const libc::c_char,
                               callback:
                                   Some(callback_minimal_broker_2 as
                                            unsafe extern "C" fn(_: *mut lws,
                                                                 _:
                                                                     lws_callback_reasons,
                                                                 _:
                                                                     *mut libc::c_void,
                                                                 _:
                                                                     *mut libc::c_void,
                                                                 _: size_t)
                                                -> libc::c_int),
                               per_session_data_size:
                                   0 as libc::c_int as size_t,
                               rx_buffer_size: 65535 as libc::c_int as size_t,
                               id: 0 as libc::c_int as libc::c_uint,
                               user:
                                   0 as *const libc::c_void as
                                       *mut libc::c_void,
                               tx_packet_size: 0,};
             init
         },
         {
             let mut init =
                 lws_protocols{name: 0 as *const libc::c_char,
                               callback: None,
                               per_session_data_size:
                                   0 as libc::c_int as size_t,
                               rx_buffer_size: 0 as libc::c_int as size_t,
                               id: 0,
                               user:
                                   0 as *const libc::c_void as
                                       *mut libc::c_void,
                               tx_packet_size: 0,};
             init
         }]
    };
unsafe extern "C" fn sigint_handler(mut sig: libc::c_int) {
    interrupted = 1 as libc::c_int;
}
#[no_mangle]
pub static mut irc_mainport_pollfd: pollfd =
    pollfd{fd: 0, events: 0, revents: 0,};
#[no_mangle]
pub static mut irc_mainport: libc::c_int = 6666 as libc::c_int;
#[no_mangle]
pub static mut rc_port: libc::c_int = 443 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn irc_mainport_bind() -> libc::c_int {
    let mut sockopt: libc::c_int = 0;
    let mut n: *mut addrinfo = 0 as *mut addrinfo;
    let mut no: *mut addrinfo = 0 as *mut addrinfo;
    let mut hostinf: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut sport: [libc::c_char; 128] = [0; 128];
    let mut r: libc::c_int = 0;
    let mut sock: libc::c_int = 0;
    snprintf(sport.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
             b"%d\x00" as *const u8 as *const libc::c_char, irc_mainport);
    memset(&mut hostinf as *mut addrinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hostinf.ai_family = 0 as libc::c_int;
    hostinf.ai_socktype = SOCK_STREAM as libc::c_int;
    hostinf.ai_flags = 0x1 as libc::c_int;
    r =
        getaddrinfo(0 as *const libc::c_char, sport.as_mut_ptr(),
                    &mut hostinf, &mut n);
    if r != 0 || n.is_null() {
        logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
             b"Cannot getaddrinfo socket %d\n\x00" as *const u8 as
                 *const libc::c_char, r);
        return -(2 as libc::c_int)
    }
    no = n;
    while !n.is_null() {
        sock = socket((*n).ai_family, (*n).ai_socktype, (*n).ai_protocol);
        if sock == -(1 as libc::c_int) {
            logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
                 b"Cannot create socket.. %d (%s)\x00" as *const u8 as
                     *const libc::c_char, *__errno_location(),
                 strerror(*__errno_location()));
            freeaddrinfo(no);
            return -(2 as libc::c_int)
        }
        sockopt = 1 as libc::c_int;
        if setsockopt(sock, 1 as libc::c_int, 2 as libc::c_int,
                      &mut sockopt as *mut libc::c_int as *mut libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) != 0 as libc::c_int {
            logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
                 b"error setting socket options (%s).\x00" as *const u8 as
                     *const libc::c_char, strerror(*__errno_location()));
            return -(1 as libc::c_int)
        }
        if bind(sock, __CONST_SOCKADDR_ARG{__sockaddr__: (*n).ai_addr,},
                (*n).ai_addrlen) == 0 as libc::c_int {
            break ;
        }
        close(sock);
        sock = -(1 as libc::c_int);
        n = (*n).ai_next
    }
    if sock == -(1 as libc::c_int) {
        logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
             b"Cannot create socket.\n\x00" as *const u8 as
                 *const libc::c_char);
        return -(3 as libc::c_int)
    }
    listen(sock, 5 as libc::c_int);
    return sock;
}
#[no_mangle]
pub unsafe extern "C" fn buff__new() -> *mut t_buff {
    let mut b: *mut t_buff = 0 as *mut t_buff;
    b =
        malloc(::std::mem::size_of::<t_buff>() as libc::c_ulong) as
            *mut t_buff;
    memset(b as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<t_buff>() as libc::c_ulong);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn buff__free(mut b: *mut t_buff) {
    free((*b).buff as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn buff__sprintf(mut fmt: *const libc::c_char,
                                       mut args: ...) -> *mut t_buff {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut b: *mut t_buff = 0 as *mut t_buff;
    b = buff__new();
    args_0 = args.clone();
    (*b).left = vasprintf(&mut (*b).buff, fmt, args_0.as_va_list());
    (*b).start = (*b).buff;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn sess_new() -> *mut t_sess {
    let mut s: *mut t_sess = 0 as *mut t_sess;
    s =
        malloc(::std::mem::size_of::<t_sess>() as libc::c_ulong) as
            *mut t_sess;
    memset(s as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<t_sess>() as libc::c_ulong);
    (*s).irc_buff_head = 0 as libc::c_int;
    (*s).irc_out_buff_tail = &mut (*s).irc_out_buff;
    (*s).rc.out_buff_tail = &mut (*s).rc.out_buff;
    (*s).next = session_list;
    session_list = s;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sess__add_irc_out(mut s: *mut t_sess,
                                           mut b: *mut t_buff) {
    *(*s).irc_out_buff_tail = b;
    (*s).irc_out_buff_tail = &mut (*b).next;
    (*(*s).poll).events =
        ((*(*s).poll).events as libc::c_int | 0x4 as libc::c_int) as
            libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn gen_id(mut id: *mut libc::c_char) {
    let mut r: libc::c_int = rand();
    snprintf(id, ::std::mem::size_of::<t_rc_id>() as libc::c_ulong,
             b"%014x\x00" as *const u8 as *const libc::c_char, r);
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_command_call(mut s: *mut t_sess,
                                               mut data: *mut libc::c_void,
                                               mut json: *mut json_object,
                                               mut fce:
                                                   Option<unsafe extern "C" fn(_:
                                                                                   *mut t_sess,
                                                                               _:
                                                                                   *mut libc::c_void,
                                                                               _:
                                                                                   *mut json_object)
                                                              -> libc::c_int>)
 -> libc::c_int {
    let mut id: t_rc_id = [0; 16];
    let mut cmd: *mut t_rc_command = 0 as *mut t_rc_command;
    gen_id(id.as_mut_ptr());
    json_object_object_add(json,
                           b"id\x00" as *const u8 as *const libc::c_char,
                           json_object_new_string(id.as_mut_ptr()));
    cmd =
        memset(malloc(::std::mem::size_of::<t_rc_command>() as libc::c_ulong),
               0 as libc::c_int,
               ::std::mem::size_of::<t_rc_command>() as libc::c_ulong) as
            *mut t_rc_command;
    sess__add_rc_out(s, -(1 as libc::c_int),
                     json_object_to_json_string(json));
    (*cmd).data = data;
    (*cmd).fce = fce;
    memcpy((*cmd).id.as_mut_ptr() as *mut libc::c_void,
           id.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<t_rc_id>() as libc::c_ulong);
    (*cmd).next = (*s).rc.commands;
    (*s).rc.commands = cmd;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_room_add(mut s: *mut t_sess,
                                           mut t: libc::c_char,
                                           mut name: *const libc::c_char,
                                           mut rid: *const libc::c_char,
                                           mut fname: *const libc::c_char,
                                           mut topic: *const libc::c_char)
 -> *mut t_rc_room {
    let mut room: *mut t_rc_room = sess__rc_room_by_rid(s, rid);
    if !room.is_null() {
        if !(*room).name.is_null() {
            free((*room).name as *mut libc::c_void);
            (*room).name = 0 as *mut libc::c_char
        }
        if !(*room).fname.is_null() {
            free((*room).fname as *mut libc::c_void);
            (*room).fname = 0 as *mut libc::c_char
        }
        if !(*room).topic.is_null() {
            free((*room).topic as *mut libc::c_void);
            (*room).topic = 0 as *mut libc::c_char
        }
        if !(*room).rid.is_null() {
            free((*room).rid as *mut libc::c_void);
            (*room).rid = 0 as *mut libc::c_char
        }
    } else {
        room =
            memset(malloc(::std::mem::size_of::<t_rc_room>() as
                              libc::c_ulong), 0 as libc::c_int,
                   ::std::mem::size_of::<t_rc_room>() as libc::c_ulong) as
                *mut t_rc_room;
        (*room).next = (*s).rc.rooms;
        (*s).rc.rooms = room
    }
    if !fname.is_null() { (*room).fname = strdup(fname) }
    if !name.is_null() { (*room).name = strdup(name) }
    if !topic.is_null() { (*room).topic = strdup(topic) }
    (*room).t = t;
    (*room).rid = strdup(rid);
    return room;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_room_by_rid(mut s: *mut t_sess,
                                              mut rid: *const libc::c_char)
 -> *mut t_rc_room {
    let mut r: *mut t_rc_room = 0 as *mut t_rc_room;
    r = (*s).rc.rooms;
    while !r.is_null() && strcmp((*r).rid, rid) != 0 { r = (*r).next }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_room_by_name(mut s: *mut t_sess,
                                               mut t: libc::c_char,
                                               mut name: *const libc::c_char)
 -> *mut t_rc_room {
    let mut r: *mut t_rc_room = 0 as *mut t_rc_room;
    r = (*s).rc.rooms;
    while !r.is_null() &&
              ((*r).t as libc::c_int != t as libc::c_int ||
                   strcmp((*r).name, name) != 0) {
        r = (*r).next
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn sess__cb_rc_createdirect(mut s: *mut t_sess,
                                                  mut data: *mut libc::c_void,
                                                  mut j: *mut json_object)
 -> libc::c_int {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut usernames: *mut json_object = 0 as *mut json_object;
    if json_read(0 as *mut libc::c_void, j,
                 b"{result:{t:%s rid:%s usernames:%o}}\x00" as *const u8 as
                     *const libc::c_char, &mut t as *mut *mut libc::c_char,
                 &mut rid as *mut *mut libc::c_char,
                 &mut usernames as *mut *mut json_object) == 3 as libc::c_int
       {
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut i: libc::c_int = 0 as libc::c_int;
        while (i as libc::c_ulong) < json_object_array_length(usernames) {
            let mut un: *mut json_object =
                json_object_array_get_idx(usernames, i as size_t);
            name = json_object_get_string(un);
            if !name.is_null() && strcmp(name, (*s).irc.nick) != 0 { break ; }
            i += 1
        }
        if !name.is_null() {
            let mut room: *mut t_rc_room =
                sess__rc_room_add(s, *t.offset(0 as libc::c_int as isize),
                                  name, rid, 0 as *const libc::c_char,
                                  0 as *const libc::c_char);
            (*room).next = (*s).rc.rooms;
            (*s).rc.rooms = room
        }
    } else if json_read(0 as *mut libc::c_void, j,
                        b"{error:{message:%s}}\x00" as *const u8 as
                            *const libc::c_char,
                        &mut msg as *mut *mut libc::c_char) ==
                  1 as libc::c_int {
        sess__add_irc_out(s,
                          buff__sprintf(b"ERROR: %s\r\n\x00" as *const u8 as
                                            *const libc::c_char, msg));
    }
    if !t.is_null() {
        free(t as *mut libc::c_void);
        t = 0 as *mut libc::c_char
    }
    if !rid.is_null() {
        free(rid as *mut libc::c_void);
        rid = 0 as *mut libc::c_char
    }
    if !msg.is_null() {
        free(msg as *mut libc::c_void);
        msg = 0 as *mut libc::c_char
    }
    sess__rc_queue_process(s);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__add_sent_message(mut s: *mut t_sess,
                                                mut id: *const libc::c_char) {
    let mut l: libc::c_int = strlen(id) as libc::c_int;
    let mut m: *mut t_rc_sndmsg =
        malloc((::std::mem::size_of::<t_rc_sndmsg>() as
                    libc::c_ulong).wrapping_add(l as
                                                    libc::c_ulong).wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
            as *mut t_rc_sndmsg;
    (*m).next = (*s).rc.sent;
    (*s).rc.sent = m;
    strcpy((*m).id.as_mut_ptr(), id);
}
#[no_mangle]
pub unsafe extern "C" fn sess__find_sent_message(mut s: *mut t_sess,
                                                 mut id: *const libc::c_char)
 -> libc::c_int {
    let mut m: *mut t_rc_sndmsg = (*s).rc.sent;
    while !m.is_null() {
        if strcmp((*m).id.as_mut_ptr(), id) == 0 { return 1 as libc::c_int }
        m = (*m).next
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__cb_rc_sendmessage(mut s: *mut t_sess,
                                                 mut data: *mut libc::c_void,
                                                 mut j: *mut json_object)
 -> libc::c_int {
    let mut _id: *mut libc::c_char = 0 as *mut libc::c_char;
    if json_read(0 as *mut libc::c_void, j,
                 b"{result:{_id:%s}}\x00" as *const u8 as *const libc::c_char,
                 &mut _id as *mut *mut libc::c_char) != 1 as libc::c_int {
        return 0 as libc::c_int
    }
    sess__add_sent_message(s, _id);
    if !_id.is_null() {
        free(_id as *mut libc::c_void);
        _id = 0 as *mut libc::c_char
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__cb_rc_getsubscriptions(mut s: *mut t_sess,
                                                      mut data:
                                                          *mut libc::c_void,
                                                      mut j: *mut json_object)
 -> libc::c_int {
    let mut sublen: libc::c_int = 0;
    logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
         b"CALLBACK - SUBS \n\x00" as *const u8 as *const libc::c_char);
    let mut res: *mut json_object = 0 as *mut json_object;
    if json_read(0 as *mut libc::c_void, j,
                 b"{result:%o}\x00" as *const u8 as *const libc::c_char,
                 &mut res as *mut *mut json_object) != 1 as libc::c_int {
        logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
             b"Malformed reply\n\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    } else {
        sublen = json_object_array_length(res) as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < sublen {
            let mut o: *mut json_object =
                json_object_array_get_idx(res, i as size_t);
            let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut rid: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
            if json_read(0 as *mut libc::c_void, o,
                         b"{fname:%s rid:%s name:%s t:%s}\x00" as *const u8 as
                             *const libc::c_char,
                         &mut fname as *mut *mut libc::c_char,
                         &mut rid as *mut *mut libc::c_char,
                         &mut name as *mut *mut libc::c_char,
                         &mut t as *mut *mut libc::c_char) != 4 as libc::c_int
               {
                logg(((1 as libc::c_int) << 0 as libc::c_int) as
                         libc::c_short,
                     b"Invalid sub record\n\x00" as *const u8 as
                         *const libc::c_char);
            } else {
                sess__rc_room_add(s, *t.offset(0 as libc::c_int as isize),
                                  name, rid, fname, 0 as *const libc::c_char);
                if !fname.is_null() {
                    free(fname as *mut libc::c_void);
                    fname = 0 as *mut libc::c_char
                }
                if !t.is_null() {
                    free(t as *mut libc::c_void);
                    t = 0 as *mut libc::c_char
                }
                if !rid.is_null() {
                    free(rid as *mut libc::c_void);
                    rid = 0 as *mut libc::c_char
                }
                if !name.is_null() {
                    free(name as *mut libc::c_void);
                    name = 0 as *mut libc::c_char
                }
            }
            i += 1
        }
        let mut r: *mut t_rc_room = (*s).rc.rooms;
        while !r.is_null() {
            if (*r).t as libc::c_int == 'c' as i32 ||
                   (*r).t as libc::c_int == 'p' as i32 {
                sess__rc_join_room(s, (*r).t, (*r).name);
            }
            r = (*r).next
        }
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn sess__cb_rc_login(mut s: *mut t_sess,
                                           mut data: *mut libc::c_void,
                                           mut j: *mut json_object)
 -> libc::c_int {
    logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
         b"CALLBACK - LOGIN\n\x00" as *const u8 as *const libc::c_char);
    let mut reason: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut error: libc::c_int = 0;
    if json_read(0 as *mut libc::c_void, j,
                 b"{error:{error:%d reason:%s}}\x00" as *const u8 as
                     *const libc::c_char, &mut error as *mut libc::c_int,
                 &mut reason as *mut *mut libc::c_char) == 2 as libc::c_int &&
           error != 0 {
        sess__add_irc_out(s,
                          buff__sprintf(b"RocketChat Error %d: %s\r\n\x00" as
                                            *const u8 as *const libc::c_char,
                                        error, reason));
        (*s).state |= 1 as libc::c_int;
        if !reason.is_null() {
            free(reason as *mut libc::c_void);
            reason = 0 as *mut libc::c_char
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_command_call_(mut s: *mut t_sess,
                                                mut data: *mut libc::c_void,
                                                mut json: *mut json_object,
                                                mut fce:
                                                    Option<unsafe extern "C" fn(_:
                                                                                    *mut t_sess,
                                                                                _:
                                                                                    *mut libc::c_void,
                                                                                _:
                                                                                    *mut json_object)
                                                               ->
                                                                   libc::c_int>)
 -> libc::c_int {
    let mut r: libc::c_int = sess__rc_command_call(s, data, json, fce);
    json_object_put(json);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_json_send_(mut s: *mut t_sess,
                                             mut json: *mut json_object)
 -> libc::c_int {
    let mut c: *const libc::c_char = json_object_to_json_string(json);
    let mut r: libc::c_int = sess__add_rc_out(s, -(1 as libc::c_int), c);
    json_object_put(json);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_start(mut s: *mut t_sess,
                                        mut ctx: *mut lws_context)
 -> libc::c_int {
    if !(*s).rc_poll.is_null() {
        logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
             b"RC session already started\x00" as *const u8 as
                 *const libc::c_char);
        return 2 as libc::c_int
    }
    (*s).rc.context = ctx;
    (*s).rc.i.context = ctx;
    (*s).rc.i.port = rc_port;
    (*s).rc.i.address = server_name;
    (*s).rc.i.path = b"/websocket\x00" as *const u8 as *const libc::c_char;
    (*s).rc.i.host = (*s).rc.i.address;
    (*s).rc.i.origin = (*s).rc.i.address;
    (*s).rc.i.opaque_user_data = s as *mut libc::c_void;
    (*s).rc.i.ssl_connection = LCCSCF_USE_SSL as libc::c_int;
    (*s).rc.i.protocol =
        b"lws-minimal-broker\x00" as *const u8 as *const libc::c_char;
    (*s).rc.i.pwsi = &mut (*s).rc.client_wsi;
    let mut ws: *mut lws = lws_client_connect_via_info(&mut (*s).rc.i);
    if ws.is_null() {
        logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
             b"Error in lws_client_connect_via_info\n\x00" as *const u8 as
                 *const libc::c_char);
        return 1 as libc::c_int
    }
    let mut fd: libc::c_int = lws_get_socket_fd(ws);
    logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
         b"FD=%d\n\x00" as *const u8 as *const libc::c_char, fd);
    (*s).rc_poll =
        &mut *pollfds.as_mut_ptr().offset((poll_addfd as
                                               unsafe extern "C" fn(_:
                                                                        libc::c_int,
                                                                    _:
                                                                        *mut t_sess,
                                                                    _:
                                                                        libc::c_int)
                                                   ->
                                                       libc::c_int)(fd, s,
                                                                    0x1 as
                                                                        libc::c_int
                                                                        |
                                                                        0x4 as
                                                                            libc::c_int)
                                              as isize) as *mut pollfd;
    sess__rc_json_send_(s,
                        json_create(0 as *mut libc::c_void,
                                    b"{msg:%s version:%s support:[%s]}\x00" as
                                        *const u8 as *const libc::c_char,
                                    b"connect\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"1\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"1\x00" as *const u8 as
                                        *const libc::c_char));
    sess__rc_command_call_(s, 0 as *mut libc::c_void,
                           json_create(0 as *mut libc::c_void,
                                       b"{msg:%s method:%s params:[{resume:%s}]}\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       b"method\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"login\x00" as *const u8 as
                                           *const libc::c_char,
                                       (*s).rc.token),
                           Some(sess__cb_rc_login as
                                    unsafe extern "C" fn(_: *mut t_sess,
                                                         _: *mut libc::c_void,
                                                         _: *mut json_object)
                                        -> libc::c_int));
    sess__rc_command_call_(s, 0 as *mut libc::c_void,
                           json_create(0 as *mut libc::c_void,
                                       b"{msg:%s method:%s params:[]}\x00" as
                                           *const u8 as *const libc::c_char,
                                       b"method\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"subscriptions/get\x00" as *const u8
                                           as *const libc::c_char),
                           Some(sess__cb_rc_getsubscriptions as
                                    unsafe extern "C" fn(_: *mut t_sess,
                                                         _: *mut libc::c_void,
                                                         _: *mut json_object)
                                        -> libc::c_int));
    sess__rc_command_call_(s, 0 as *mut libc::c_void,
                           json_create(0 as *mut libc::c_void,
                                       b"{msg:%s name:%s params:[%s %b]}\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       b"sub\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"stream-room-messages\x00" as
                                           *const u8 as *const libc::c_char,
                                       b"__my_messages__\x00" as *const u8 as
                                           *const libc::c_char,
                                       0 as libc::c_int), None);
    (*s).rc.context = lws_get_context(ws);
    (*s).rc.protocol = lws_get_protocol(ws);
    (*s).rc.vhost = lws_get_vhost(ws);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__irc_nick_set(mut s: *mut t_sess,
                                            mut newnick: *const libc::c_char)
 -> libc::c_int {
    sess__add_irc_out(s,
                      buff__sprintf(b":%s NICK %s\r\n\x00" as *const u8 as
                                        *const libc::c_char, (*s).irc.nick,
                                    newnick));
    if !(*s).irc.nick.is_null() {
        free((*s).irc.nick as *mut libc::c_void);
        (*s).irc.nick = 0 as *mut libc::c_char
    }
    (*s).irc.nick = strdup(newnick);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__irc_send_message(mut s: *mut t_sess,
                                                mut t: libc::c_char,
                                                mut srcname:
                                                    *const libc::c_char,
                                                mut name: *const libc::c_char,
                                                mut pfx: *mut libc::c_char,
                                                mut msg: *mut libc::c_char)
 -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut pmsg: [libc::c_char; 2] = [0; 2];
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    if t as libc::c_int == 'c' as i32 {
        pmsg[0 as libc::c_int as usize] = '#' as i32 as libc::c_char;
        pmsg[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
    } else {
        pmsg[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
    }
    start = msg;
    loop  {
        next = strchr(start, '\n' as i32);
        if !next.is_null() { *next = '\u{0}' as i32 as libc::c_char }
        if pfx.is_null() {
            sess__add_irc_out(s,
                              buff__sprintf(b":%s PRIVMSG %s%s :%s\r\n\x00" as
                                                *const u8 as
                                                *const libc::c_char, srcname,
                                            pmsg.as_mut_ptr(), name, start));
        } else {
            sess__add_irc_out(s,
                              buff__sprintf(b":%s PRIVMSG %s%s :\x02%s\x0f %s\r\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, srcname,
                                            pmsg.as_mut_ptr(), name, pfx,
                                            start));
        }
        if next.is_null() || *next.offset(1 as libc::c_int as isize) == 0 {
            break ;
        }
        start = next.offset(1 as libc::c_int as isize)
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn sess__cb_rc_getusers(mut s: *mut t_sess,
                                              mut data: *mut libc::c_void,
                                              mut j: *mut json_object)
 -> libc::c_int {
    let mut r: *mut t_rc_room = data as *mut t_rc_room;
    let mut buff: [libc::c_char; 512] = [0; 512];
    let mut b: *mut libc::c_char = buff.as_mut_ptr();
    buff[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    let mut records: *mut json_object = 0 as *mut json_object;
    if json_read(0 as *mut libc::c_void, j,
                 b"{result:{records:%o}}\x00" as *const u8 as
                     *const libc::c_char,
                 &mut records as *mut *mut json_object) != 1 as libc::c_int {
        return 0 as libc::c_int
    }
    let mut cnt_args: libc::c_int =
        json_object_array_length(records) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < cnt_args {
        let mut n: *mut json_object = 0 as *mut json_object;
        let mut p: *mut json_object =
            json_object_array_get_idx(records, i as size_t);
        n =
            json_object_object_get(p,
                                   b"username\x00" as *const u8 as
                                       *const libc::c_char);
        if !n.is_null() {
            let mut st: *const libc::c_char = json_object_get_string(n);
            b =
                stpncpy(b, st,
                        buff.as_mut_ptr().offset(::std::mem::size_of::<[libc::c_char; 512]>()
                                                     as libc::c_ulong as
                                                     isize).wrapping_offset_from(b)
                            as libc::c_long as libc::c_ulong);
            b =
                stpncpy(b, b" \x00" as *const u8 as *const libc::c_char,
                        buff.as_mut_ptr().offset(::std::mem::size_of::<[libc::c_char; 512]>()
                                                     as libc::c_ulong as
                                                     isize).wrapping_offset_from(b)
                            as libc::c_long as libc::c_ulong)
        }
        i += 1
    }
    *b = 0 as libc::c_int as libc::c_char;
    sess__add_irc_out(s,
                      buff__sprintf(b":%s 353 %s = #%s :%s\r\n\x00" as
                                        *const u8 as *const libc::c_char,
                                    selfident, (*s).irc.nick, (*r).name,
                                    buff.as_mut_ptr()));
    sess__add_irc_out(s,
                      buff__sprintf(b":%s 366 %s #%s :End of NAMES list\r\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    selfident, (*s).irc.nick, (*r).name));
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__cb_rc_joinroom(mut s: *mut t_sess,
                                              mut data: *mut libc::c_void,
                                              mut j: *mut json_object)
 -> libc::c_int {
    let mut r: *mut t_rc_room = data as *mut t_rc_room;
    sess__add_irc_out(s,
                      buff__sprintf(b":%s JOIN #%s\r\n\x00" as *const u8 as
                                        *const libc::c_char, (*s).irc.nick,
                                    (*r).name));
    sess__add_irc_out(s,
                      buff__sprintf(b":%s 332 %s #%s :%s\r\n\x00" as *const u8
                                        as *const libc::c_char, selfident,
                                    (*s).irc.nick, (*r).name,
                                    if !(*r).topic.is_null() {
                                        (*r).topic
                                    } else {
                                        b"\x00" as *const u8 as
                                            *const libc::c_char
                                    }));
    sess__rc_command_call_(s, r as *mut libc::c_void,
                           json_create(0 as *mut libc::c_void,
                                       b"{msg:%s method:%s params:[%s %b {limit:%d skip:%d} %s]}\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       b"method\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"getUsersOfRoom\x00" as *const u8 as
                                           *const libc::c_char, (*r).rid,
                                       0 as libc::c_int, 100 as libc::c_int,
                                       0 as libc::c_int,
                                       b"\x00" as *const u8 as
                                           *const libc::c_char),
                           Some(sess__cb_rc_getusers as
                                    unsafe extern "C" fn(_: *mut t_sess,
                                                         _: *mut libc::c_void,
                                                         _: *mut json_object)
                                        -> libc::c_int));
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__cb_rc_getroom(mut s: *mut t_sess,
                                             mut data: *mut libc::c_void,
                                             mut j: *mut json_object)
 -> libc::c_int {
    let mut room: *mut t_rc_room = 0 as *mut t_rc_room;
    let mut r: libc::c_int = 0;
    let mut _id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut topic: *mut libc::c_char = 0 as *mut libc::c_char;
    r =
        json_read(0 as *mut libc::c_void, j,
                  b"{result:{_id:%s name:%s fname:%s t:%s topic?%s}}\x00" as
                      *const u8 as *const libc::c_char,
                  &mut _id as *mut *mut libc::c_char,
                  &mut name as *mut *mut libc::c_char,
                  &mut fname as *mut *mut libc::c_char,
                  &mut t as *mut *mut libc::c_char,
                  &mut topic as *mut *mut libc::c_char);
    if !(r < 4 as libc::c_int) {
        room = sess__rc_room_add(s, *t, name, _id, fname, topic);
        sess__rc_command_call_(s, room as *mut libc::c_void,
                               json_create(0 as *mut libc::c_void,
                                           b"{msg:%s method:%s params:[%s %o]}\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           b"method\x00" as *const u8 as
                                               *const libc::c_char,
                                           b"joinRoom\x00" as *const u8 as
                                               *const libc::c_char, _id,
                                           0 as *mut libc::c_void),
                               Some(sess__cb_rc_joinroom as
                                        unsafe extern "C" fn(_: *mut t_sess,
                                                             _:
                                                                 *mut libc::c_void,
                                                             _:
                                                                 *mut json_object)
                                            -> libc::c_int));
    }
    if !_id.is_null() {
        free(_id as *mut libc::c_void);
        _id = 0 as *mut libc::c_char
    }
    if !t.is_null() {
        free(t as *mut libc::c_void);
        t = 0 as *mut libc::c_char
    }
    if !name.is_null() {
        free(name as *mut libc::c_void);
        name = 0 as *mut libc::c_char
    }
    if !fname.is_null() {
        free(fname as *mut libc::c_void);
        fname = 0 as *mut libc::c_char
    }
    if !topic.is_null() {
        free(topic as *mut libc::c_void);
        topic = 0 as *mut libc::c_char
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_join_room(mut s: *mut t_sess,
                                            mut t: libc::c_char,
                                            mut name: *const libc::c_char)
 -> libc::c_int {
    let mut tt: [libc::c_char; 2] = [0; 2];
    tt[0 as libc::c_int as usize] = t;
    tt[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if t as libc::c_int == 'd' as i32 {
        sess__rc_command_call_(s, 0 as *mut libc::c_void,
                               json_create(0 as *mut libc::c_void,
                                           b"{msg:%s method:%s params:[%s]}\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           b"method\x00" as *const u8 as
                                               *const libc::c_char,
                                           b"createDirectMessage\x00" as
                                               *const u8 as
                                               *const libc::c_char, name),
                               Some(sess__cb_rc_createdirect as
                                        unsafe extern "C" fn(_: *mut t_sess,
                                                             _:
                                                                 *mut libc::c_void,
                                                             _:
                                                                 *mut json_object)
                                            -> libc::c_int));
    } else {
        sess__rc_command_call_(s, 0 as *mut libc::c_void,
                               json_create(0 as *mut libc::c_void,
                                           b"{msg:%s method:%s params:[%s %s]}\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           b"method\x00" as *const u8 as
                                               *const libc::c_char,
                                           b"getRoomByTypeAndName\x00" as
                                               *const u8 as
                                               *const libc::c_char,
                                           tt.as_mut_ptr(),
                                           if *name as libc::c_int ==
                                                  '#' as i32 {
                                               name.offset(1 as libc::c_int as
                                                               isize)
                                           } else { name }),
                               Some(sess__cb_rc_getroom as
                                        unsafe extern "C" fn(_: *mut t_sess,
                                                             _:
                                                                 *mut libc::c_void,
                                                             _:
                                                                 *mut json_object)
                                            -> libc::c_int));
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_queue_message(mut s: *mut t_sess,
                                                mut t: libc::c_char,
                                                mut name: *const libc::c_char,
                                                mut msgs: *const libc::c_char)
 -> libc::c_int {
    let mut msg: *mut t_rc_message = 0 as *mut t_rc_message;
    let mut m: *mut *mut t_rc_message = 0 as *mut *mut t_rc_message;
    m = &mut (*s).rc.messages;
    while !(*m).is_null() { m = &mut (**m).next }
    msg =
        memset(malloc(::std::mem::size_of::<t_rc_message>() as libc::c_ulong),
               0 as libc::c_int,
               ::std::mem::size_of::<t_rc_message>() as libc::c_ulong) as
            *mut t_rc_message;
    (*msg).t = t;
    (*msg).dstname = strdup(name);
    (*msg).msg = strdup(msgs);
    *m = msg;
    return 0 as libc::c_int;
}
unsafe extern "C" fn rc_message_cmpid(mut a: *const t_rc_message,
                                      mut b: *const t_rc_message)
 -> libc::c_int {
    return strcmp((*a).id, (*b).id);
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_add_message(mut s: *mut t_sess,
                                              mut id: *const libc::c_char,
                                              mut msg: *const libc::c_char,
                                              mut sender: *const libc::c_char,
                                              mut reactions:
                                                  *const libc::c_char)
 -> *mut t_rc_message {
    let mut m: *mut t_rc_message = 0 as *mut t_rc_message;
    m =
        memset(malloc(::std::mem::size_of::<t_rc_message>() as libc::c_ulong),
               0 as libc::c_int,
               ::std::mem::size_of::<t_rc_message>() as libc::c_ulong) as
            *mut t_rc_message;
    (*m).id = strdup(id);
    (*m).msg =
        if !msg.is_null() { strdup(msg) } else { 0 as *mut libc::c_char };
    (*m).sender =
        if !sender.is_null() {
            strdup(sender)
        } else { 0 as *mut libc::c_char };
    (*m).reactions =
        if !reactions.is_null() {
            strdup(reactions)
        } else { 0 as *mut libc::c_char };
    (*m).next = (*s).rc.in_messages;
    (*s).rc.in_messages = m;
    tsearch(m as *mut libc::c_void,
            &mut (*s).rc.in_messages_tree as *mut *mut libc::c_void,
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *const t_rc_message,
                                                                _:
                                                                    *const t_rc_message)
                                               -> libc::c_int>,
                                    Option<unsafe extern "C" fn(_:
                                                                    *const libc::c_void,
                                                                _:
                                                                    *const libc::c_void)
                                               ->
                                                   libc::c_int>>(Some(rc_message_cmpid
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *const t_rc_message,
                                                                                               _:
                                                                                                   *const t_rc_message)
                                                                              ->
                                                                                  libc::c_int)));
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_find_message(mut s: *mut t_sess,
                                               mut id: *const libc::c_char)
 -> *mut t_rc_message {
    let mut m: *mut *mut t_rc_message = 0 as *mut *mut t_rc_message;
    let mut fm: t_rc_message =
        t_rc_message{id: 0 as *const libc::c_char,
                     t: 0,
                     dstname: 0 as *mut libc::c_char,
                     msg: 0 as *mut libc::c_char,
                     sender: 0 as *mut libc::c_char,
                     reactions: 0 as *mut libc::c_char,
                     tim: 0,
                     next: 0 as *mut t_rc_message,};
    fm.id = id;
    m =
        tfind(&mut fm as *mut t_rc_message as *mut libc::c_void,
              &mut (*s).rc.in_messages_tree as *mut *mut libc::c_void,
              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                      *const t_rc_message,
                                                                  _:
                                                                      *const t_rc_message)
                                                 -> libc::c_int>,
                                      Option<unsafe extern "C" fn(_:
                                                                      *const libc::c_void,
                                                                  _:
                                                                      *const libc::c_void)
                                                 ->
                                                     libc::c_int>>(Some(rc_message_cmpid
                                                                            as
                                                                            unsafe extern "C" fn(_:
                                                                                                     *const t_rc_message,
                                                                                                 _:
                                                                                                     *const t_rc_message)
                                                                                ->
                                                                                    libc::c_int)))
            as *mut *mut t_rc_message;
    if !m.is_null() { return *m } else { return 0 as *mut t_rc_message };
}
#[no_mangle]
pub unsafe extern "C" fn rc_message__free(mut m: *mut t_rc_message) {
    if !(*m).dstname.is_null() {
        free((*m).dstname as *mut libc::c_void);
        (*m).dstname = 0 as *mut libc::c_char
    }
    if !(*m).reactions.is_null() {
        free((*m).reactions as *mut libc::c_void);
        (*m).reactions = 0 as *mut libc::c_char
    }
    if !(*m).msg.is_null() {
        free((*m).msg as *mut libc::c_void);
        (*m).msg = 0 as *mut libc::c_char
    }
    if !m.is_null() {
        free(m as *mut libc::c_void);
        m = 0 as *mut t_rc_message
    };
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_queue_process(mut s: *mut t_sess)
 -> libc::c_int {
    let mut msg: *mut t_rc_message = 0 as *mut t_rc_message;
    let mut m: *mut *mut t_rc_message = 0 as *mut *mut t_rc_message;
    m = &mut (*s).rc.messages;
    loop  {
        msg = *m;
        if msg.is_null() { break ; }
        if sess__rc_send_message(s, (*msg).t, (*msg).dstname, (*msg).msg) == 0
           {
            *m = (**m).next;
            rc_message__free(msg);
        }
        m = &mut (**m).next
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_send_message(mut s: *mut t_sess,
                                               mut t: libc::c_char,
                                               mut name: *const libc::c_char,
                                               mut msg: *const libc::c_char)
 -> libc::c_int {
    let mut room: *mut t_rc_room = 0 as *mut t_rc_room;
    room = sess__rc_room_by_name(s, t, name);
    if room.is_null() && t as libc::c_int == 'c' as i32 {
        room = sess__rc_room_by_name(s, 'p' as i32 as libc::c_char, name)
    }
    if room.is_null() {
        logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
             b"Room %c:%s NOT FOUND, not supported atm\n\x00" as *const u8 as
                 *const libc::c_char, t as libc::c_int, name);
        return -(1 as libc::c_int)
    }
    sess__rc_command_call_(s, 0 as *mut libc::c_void,
                           json_create(0 as *mut libc::c_void,
                                       b"{msg:%s method:%s params:[{rid:%s msg:%s}]}\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       b"method\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"sendMessage\x00" as *const u8 as
                                           *const libc::c_char, (*room).rid,
                                       msg),
                           Some(sess__cb_rc_sendmessage as
                                    unsafe extern "C" fn(_: *mut t_sess,
                                                         _: *mut libc::c_void,
                                                         _: *mut json_object)
                                        -> libc::c_int));
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_set_away(mut s: *mut t_sess,
                                           mut msg: *const libc::c_char)
 -> libc::c_int {
    if msg.is_null() {
        logg(((1 as libc::c_int) << 3 as libc::c_int) as libc::c_short,
             b"Setting back\n\x00" as *const u8 as *const libc::c_char);
        sess__rc_set_back(s);
    } else {
        logg(((1 as libc::c_int) << 3 as libc::c_int) as libc::c_short,
             b"Setting away\n\x00" as *const u8 as *const libc::c_char);
        sess__rc_command_call_(s, 0 as *mut libc::c_void,
                               json_create(0 as *mut libc::c_void,
                                           b"{msg:%s method:%s params:[%s]}\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           b"method\x00" as *const u8 as
                                               *const libc::c_char,
                                           b"setUserStatus\x00" as *const u8
                                               as *const libc::c_char,
                                           b"away\x00" as *const u8 as
                                               *const libc::c_char),
                               Some(sess__cb_rc_set_away as
                                        unsafe extern "C" fn(_: *mut t_sess,
                                                             _:
                                                                 *mut libc::c_void,
                                                             _:
                                                                 *mut json_object)
                                            -> libc::c_int));
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__rc_set_back(mut s: *mut t_sess)
 -> libc::c_int {
    sess__rc_command_call_(s, 0 as *mut libc::c_void,
                           json_create(0 as *mut libc::c_void,
                                       b"{msg:%s method:%s params:[%s]}\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       b"method\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"setUserStatus\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"online\x00" as *const u8 as
                                           *const libc::c_char),
                           Some(sess__cb_rc_set_back as
                                    unsafe extern "C" fn(_: *mut t_sess,
                                                         _: *mut libc::c_void,
                                                         _: *mut json_object)
                                        -> libc::c_int));
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__cb_rc_set_away(mut s: *mut t_sess,
                                              mut data: *mut libc::c_void,
                                              mut j: *mut json_object)
 -> libc::c_int {
    let mut buff: [libc::c_char; 512] = [0; 512];
    sess__add_irc_out(s,
                      buff__sprintf(b":%s 306 :You have been marked as being away\r\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    selfident, buff.as_mut_ptr()));
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__cb_rc_set_back(mut s: *mut t_sess,
                                              mut data: *mut libc::c_void,
                                              mut j: *mut json_object)
 -> libc::c_int {
    let mut buff: [libc::c_char; 512] = [0; 512];
    sess__add_irc_out(s,
                      buff__sprintf(b":%s 305 :You are no longer marked as being away\r\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    selfident, buff.as_mut_ptr()));
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__free(mut s: *mut t_sess) -> libc::c_int {
    let mut d: *mut *mut t_sess = 0 as *mut *mut t_sess;
    d = &mut session_list;
    while !(*d).is_null() && *d != s { d = &mut (**d).next }
    *d = (*s).next;
    tdestroy((*s).rc.in_messages_tree,
             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                     *mut t_rc_message)
                                                -> ()>,
                                     Option<unsafe extern "C" fn(_:
                                                                     *mut libc::c_void)
                                                ->
                                                    ()>>(Some(rc_message__free
                                                                  as
                                                                  unsafe extern "C" fn(_:
                                                                                           *mut t_rc_message)
                                                                      ->
                                                                          ())));
    free(s as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sess__close(mut s: *mut t_sess) -> libc::c_int {
    logg(((1 as libc::c_int) << 1 as libc::c_int) as libc::c_short,
         b"Closing session FDs %d,%d\n\x00" as *const u8 as
             *const libc::c_char, (*(*s).poll).fd,
         if !(*s).rc_poll.is_null() {
             (*(*s).rc_poll).fd
         } else { -(1 as libc::c_int) });
    shutdown((*(*s).poll).fd, SHUT_RDWR as libc::c_int);
    close((*(*s).poll).fd);
    (*s).rc.finished = 1 as libc::c_int as libc::c_char;
    if !(*s).rc_poll.is_null() {
        shutdown((*(*s).rc_poll).fd, SHUT_RDWR as libc::c_int);
        close((*(*s).rc_poll).fd);
        lws_service_fd((*s).rc.context, (*s).rc_poll);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= maxfds {
        if &mut *pollfds.as_mut_ptr().offset(i as isize) as *mut pollfd ==
               (*s).poll {
            poll_removefd(i);
            break ;
        } else { i += 1 }
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 <= maxfds {
        if &mut *pollfds.as_mut_ptr().offset(i_0 as isize) as *mut pollfd ==
               (*s).rc_poll {
            poll_removefd(i_0);
            break ;
        } else { i_0 += 1 }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn irc__process(mut s: *mut t_sess,
                                      mut ctx: *mut lws_context)
 -> libc::c_int {
    let mut current_block: u64;
    let mut st: libc::c_int = 0 as libc::c_int;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
    c = (*s).irc_buff.as_mut_ptr();
    while c < (*s).irc_buff.as_mut_ptr().offset((*s).irc_buff_head as isize)
              && st != 2 as libc::c_int {
        match *c as libc::c_int {
            13 => { st = 1 as libc::c_int }
            10 => { st = 2 as libc::c_int }
            _ => { st = 0 as libc::c_int }
        }
        if st == 2 as libc::c_int { break ; }
        c = c.offset(1)
    }
    if st != 2 as libc::c_int { return 1 as libc::c_int }
    if c > (*s).irc_buff.as_mut_ptr() &&
           *c.offset(-(1 as libc::c_int as isize)) as libc::c_int ==
               '\r' as i32 {
        *c.offset(-(1 as libc::c_int as isize)) =
            '\u{0}' as i32 as libc::c_char
    } else { *c = '\u{0}' as i32 as libc::c_char }
    end = c;
    logg(((1 as libc::c_int) << 2 as libc::c_int) as libc::c_short,
         b"IRC received: %s\n\x00" as *const u8 as *const libc::c_char,
         (*s).irc_buff.as_mut_ptr());
    b = (*s).irc_buff.as_mut_ptr();
    c = strchr(b, ' ' as i32);
    if c.is_null() {
        logg(((1 as libc::c_int) << 3 as libc::c_int) as libc::c_short,
             b"IRC command without parameter\n\x00" as *const u8 as
                 *const libc::c_char);
        current_block = 5570520647587228887;
    } else {
        *c = '\u{0}' as i32 as libc::c_char;
        if *b as libc::c_int == ':' as i32 {
            c = strchr(c.offset(1 as libc::c_int as isize), ' ' as i32);
            if c.is_null() {
                logg(((1 as libc::c_int) << 0 as libc::c_int) as
                         libc::c_short,
                     b"Can\'t find #2 space\n\x00" as *const u8 as
                         *const libc::c_char);
                current_block = 2977767453398940484;
            } else {
                b = c.offset(1 as libc::c_int as isize);
                *c = '\u{0}' as i32 as libc::c_char;
                current_block = 17788412896529399552;
            }
        } else { current_block = 17788412896529399552; }
        match current_block {
            2977767453398940484 => { }
            _ => {
                c = c.offset(1 as libc::c_int as isize);
                current_block = 5570520647587228887;
            }
        }
    }
    match current_block {
        5570520647587228887 => {
            command = b;
            logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
                 b"command \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                 command);
            if !c.is_null() &&
                   strcmp(command,
                          b"NICK\x00" as *const u8 as *const libc::c_char) ==
                       0 {
                if (*s).irc.nick.is_null() || strcmp((*s).irc.nick, c) == 0 {
                    sess__add_irc_out(s,
                                      buff__sprintf(b":%s 001 %s :Welcome to the Internet Relay Network %s\r\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    selfident, c, c));
                    sess__add_irc_out(s,
                                      buff__sprintf(b":myserver 002 alnovak Your host is localhost, running version 1\r\n:myserver 003 alnovak This server was created Tue Oct 22, 16:00:54 UTC\r\n:myserver 004 alnovak localhost 1 oirw abeIiklmnopqstv\r\n:%s 375 %s :%s message of the day\r\n:%s 372 %s :RocketChat->IRC gateway!\r\n:%s 376 %s :End of message of the day.\r\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    selfident, c, selfident,
                                                    selfident, c, selfident,
                                                    c));
                    if !(*s).irc.nick.is_null() {
                        free((*s).irc.nick as *mut libc::c_void);
                        (*s).irc.nick = 0 as *mut libc::c_char
                    }
                    (*s).irc.nick = strdup(c)
                }
            } else if !(strcmp(command,
                               b"USER\x00" as *const u8 as
                                   *const libc::c_char) == 0) {
                if !c.is_null() &&
                       strcmp(command,
                              b"PING\x00" as *const u8 as *const libc::c_char)
                           == 0 {
                    sess__add_irc_out(s,
                                      buff__sprintf(b":%s PONG %s :%s\r\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    selfident, selfident, c));
                } else if !c.is_null() &&
                              (strcmp(command,
                                      b"PASS\x00" as *const u8 as
                                          *const libc::c_char) == 0 ||
                                   strcmp(command,
                                          b"IDENTIFY\x00" as *const u8 as
                                              *const libc::c_char) == 0) {
                    if !(*s).rc.token.is_null() {
                        free((*s).rc.token as *mut libc::c_void);
                        (*s).rc.token = 0 as *mut libc::c_char
                    }
                    if *c as libc::c_int == ':' as i32 { c = c.offset(1) }
                    (*s).rc.token = strdup(c);
                    sess__rc_start(s, ctx);
                } else if !c.is_null() &&
                              strcmp(command,
                                     b"PRIVMSG\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                    let mut msg: *mut libc::c_char = strchr(c, ' ' as i32);
                    let mut t: libc::c_char = 0;
                    logg(((1 as libc::c_int) << 3 as libc::c_int) as
                             libc::c_short,
                         b"PRIVMSG->msg = %p,c = %s\n\x00" as *const u8 as
                             *const libc::c_char, msg, c);
                    if !msg.is_null() {
                        let fresh1 = msg;
                        msg = msg.offset(1);
                        *fresh1 = '\u{0}' as i32 as libc::c_char;
                        if *msg as libc::c_int == ':' as i32 {
                            msg = msg.offset(1)
                        }
                        t =
                            if *c as libc::c_int == '#' as i32 {
                                'c' as i32
                            } else { 'd' as i32 } as libc::c_char;
                        if t as libc::c_int == 'c' as i32 { c = c.offset(1) }
                        if sess__rc_send_message(s, t, c, msg) ==
                               -(1 as libc::c_int) {
                            sess__rc_join_room(s, t, c);
                            sess__rc_queue_message(s, t, c, msg);
                        }
                    }
                } else if !(strcmp(command,
                                   b"WHO\x00" as *const u8 as
                                       *const libc::c_char) == 0) {
                    if !c.is_null() &&
                           strcmp(command,
                                  b"JOIN\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                        sess__rc_join_room(s, 'c' as i32 as libc::c_char, c);
                    } else if strcmp(command,
                                     b"AWAY\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        sess__rc_set_away(s, c);
                    } else if strcmp(command,
                                     b"QUERY\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        sess__rc_join_room(s, 'd' as i32 as libc::c_char, c);
                    } else if strcmp(command,
                                     b"QUIT\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        shutdown((*(*s).poll).fd, SHUT_RDWR as libc::c_int);
                        shutdown((*(*s).rc_poll).fd,
                                 SHUT_RDWR as libc::c_int);
                    } else {
                        logg(((1 as libc::c_int) << 0 as libc::c_int) as
                                 libc::c_short,
                             b"Unrecognized command %s\n\x00" as *const u8 as
                                 *const libc::c_char, command);
                    }
                }
            }
        }
        _ => { }
    }
    memcpy((*s).irc_buff.as_mut_ptr() as *mut libc::c_void,
           end.offset(1 as libc::c_int as isize) as *const libc::c_void,
           (*s).irc_buff.as_mut_ptr().offset((*s).irc_buff_head as
                                                 isize).wrapping_offset_from(end.offset(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize))
               as libc::c_long as libc::c_ulong);
    (*s).irc_buff_head =
        ((*s).irc_buff_head as libc::c_long -
             (end.wrapping_offset_from((*s).irc_buff.as_mut_ptr()) as
                  libc::c_long + 1 as libc::c_int as libc::c_long)) as
            libc::c_int;
    return 0 as libc::c_int;
}
static mut doc_usage: *const libc::c_char =
    b"Usage: %s [options] <RC server hostname>\nOptions:\n\t-h\tshows this help\n\t-d\tincreases logging verbosity, can be used repeatedly\n\t-l\tIRC listen port (%d)\n\t-p\tRC server port (%d)\n\n\x00"
        as *const u8 as *const libc::c_char;
fn main_0(args: HashSet<String>) -> i32 {
    let mut info: lws_context_creation_info =
        lws_context_creation_info{port: 0,
                                  iface: 0 as *const libc::c_char,
                                  protocols: 0 as *const lws_protocols,
                                  extensions: 0 as *const lws_extension,
                                  token_limits: 0 as *const lws_token_limits,
                                  ssl_private_key_password:
                                      0 as *const libc::c_char,
                                  ssl_cert_filepath: 0 as *const libc::c_char,
                                  ssl_private_key_filepath:
                                      0 as *const libc::c_char,
                                  ssl_ca_filepath: 0 as *const libc::c_char,
                                  ssl_cipher_list: 0 as *const libc::c_char,
                                  http_proxy_address:
                                      0 as *const libc::c_char,
                                  http_proxy_port: 0,
                                  gid: 0,
                                  uid: 0,
                                  options: 0,
                                  user: 0 as *mut libc::c_void,
                                  ka_time: 0,
                                  ka_probes: 0,
                                  ka_interval: 0,
                                  provided_client_ssl_ctx: 0 as *mut SSL_CTX,
                                  max_http_header_data: 0,
                                  max_http_header_pool: 0,
                                  count_threads: 0,
                                  fd_limit_per_thread: 0,
                                  timeout_secs: 0,
                                  ecdh_curve: 0 as *const libc::c_char,
                                  vhost_name: 0 as *const libc::c_char,
                                  plugin_dirs:
                                      0 as *const *const libc::c_char,
                                  pvo: 0 as *const lws_protocol_vhost_options,
                                  keepalive_timeout: 0,
                                  log_filepath: 0 as *const libc::c_char,
                                  mounts: 0 as *const lws_http_mount,
                                  server_string: 0 as *const libc::c_char,
                                  pt_serv_buf_size: 0,
                                  max_http_header_data2: 0,
                                  ssl_options_set: 0,
                                  ssl_options_clear: 0,
                                  ws_ping_pong_interval: 0,
                                  headers:
                                      0 as *const lws_protocol_vhost_options,
                                  reject_service_keywords:
                                      0 as *const lws_protocol_vhost_options,
                                  external_baggage_free_on_destroy:
                                      0 as *mut libc::c_void,
                                  client_ssl_private_key_password:
                                      0 as *const libc::c_char,
                                  client_ssl_cert_filepath:
                                      0 as *const libc::c_char,
                                  client_ssl_cert_mem:
                                      0 as *const libc::c_void,
                                  client_ssl_cert_mem_len: 0,
                                  client_ssl_private_key_filepath:
                                      0 as *const libc::c_char,
                                  client_ssl_ca_filepath:
                                      0 as *const libc::c_char,
                                  client_ssl_ca_mem: 0 as *const libc::c_void,
                                  client_ssl_ca_mem_len: 0,
                                  client_ssl_cipher_list:
                                      0 as *const libc::c_char,
                                  fops: 0 as *const lws_plat_file_ops,
                                  simultaneous_ssl_restriction: 0,
                                  socks_proxy_address:
                                      0 as *const libc::c_char,
                                  socks_proxy_port: 0,
                                  bind_iface: 0,
                                  ssl_info_event_mask: 0,
                                  timeout_secs_ah_idle: 0,
                                  ip_limit_ah: 0,
                                  ip_limit_wsi: 0,
                                  http2_settings: [0; 7],
                                  error_document_404:
                                      0 as *const libc::c_char,
                                  alpn: 0 as *const libc::c_char,
                                  foreign_loops: 0 as *mut *mut libc::c_void,
                                  signal_cb: None,
                                  pcontext: 0 as *mut *mut lws_context,
                                  finalize: None,
                                  finalize_arg: 0 as *mut libc::c_void,
                                  max_http_header_pool2: 0,
                                  ssl_client_options_set: 0,
                                  ssl_client_options_clear: 0,
                                  tls1_3_plus_cipher_list:
                                      0 as *const libc::c_char,
                                  client_tls_1_3_plus_cipher_list:
                                      0 as *const libc::c_char,
                                  listen_accept_role:
                                      0 as *const libc::c_char,
                                  listen_accept_protocol:
                                      0 as *const libc::c_char,
                                  pprotocols: 0 as *mut *const lws_protocols,
                                  server_ssl_cert_mem:
                                      0 as *const libc::c_void,
                                  server_ssl_cert_mem_len: 0,
                                  server_ssl_private_key_mem:
                                      0 as *const libc::c_void,
                                  server_ssl_private_key_mem_len: 0,
                                  server_ssl_ca_mem: 0 as *const libc::c_void,
                                  server_ssl_ca_mem_len: 0,
                                  username: 0 as *const libc::c_char,
                                  groupname: 0 as *const libc::c_char,
                                  unix_socket_perms: 0 as *const libc::c_char,
                                  system_ops: 0 as *const lws_system_ops_t,
                                  detailed_latency_cb: None,
                                  detailed_latency_filepath:
                                      0 as *const libc::c_char,
                                  retry_and_idle_policy:
                                      0 as *const lws_retry_bo_t,
                                  register_notifier_list:
                                      0 as
                                          *const *mut lws_state_notify_link_t,
                                  udp_loss_sim_tx_pc: 0,
                                  udp_loss_sim_rx_pc: 0,
                                  _unused: [0 as *mut libc::c_void; 2],};
    let mut context: *mut lws_context = 0 as *mut lws_context;
    let mut showhelp: libc::c_int = 0 as libc::c_int;
    let mut logmask: libc::c_int = 1 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut logs: libc::c_int =
        (1 as libc::c_int) << 10 as libc::c_int |
            (1 as libc::c_int) << 0 as libc::c_int |
            (1 as libc::c_int) << 1 as libc::c_int |
            (1 as libc::c_int) << 2 as libc::c_int;
    loop  {
        c =
            getopt(argc, argv,
                   b"hdl:p:\x00" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) { break ; }
        match c {
            104 => { showhelp = 1 as libc::c_int }
            100 => { logmask += 1 }
            108 => { irc_mainport = atoi(optarg) }
            112 => { rc_port = atoi(optarg) }
            63 => {
                logg(((1 as libc::c_int) << 0 as libc::c_int) as
                         libc::c_short,
                     b"Unknown argument \'%c\'\n\x00" as *const u8 as
                         *const libc::c_char, optopt);
                showhelp = 2 as libc::c_int
            }
            _ => {
                logg(((1 as libc::c_int) << 0 as libc::c_int) as
                         libc::c_short,
                     b"getopt error (%d,%s)\n\x00" as *const u8 as
                         *const libc::c_char, optopt,
                     if !optarg.is_null() {
                         optarg
                     } else {
                         b"<null>\x00" as *const u8 as *const libc::c_char
                     });
            }
        }
    }
    if argc < 2 as libc::c_int || argc != optind + 1 as libc::c_int ||
           showhelp != 0 {
        fprintf(stderr, doc_usage, *argv.offset(0 as libc::c_int as isize),
                irc_mainport, rc_port);
        return 1 as libc::c_int
    } else { server_name = *argv.offset(optind as isize) }
    logg_setmask(((1 as libc::c_int) << logmask) - 1 as libc::c_int);
    signal(2 as libc::c_int,
           Some(sigint_handler as
                    unsafe extern "C" fn(_: libc::c_int) -> ()));
    lws_set_log_level(logs, None);
    memset(&mut info as *mut lws_context_creation_info as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<lws_context_creation_info>() as
               libc::c_ulong);
    info.port = -(1 as libc::c_int);
    info.options = ((1 as libc::c_longlong) << 12 as libc::c_int) as uint64_t;
    info.protocols = protocols.as_ptr();
    info.fd_limit_per_thread = 128 as libc::c_int as libc::c_uint;
    memset(pollfds.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[pollfd; 512]>() as libc::c_ulong);
    memset(sessions.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<*mut t_sess>() as
                libc::c_ulong).wrapping_mul(512 as libc::c_int as
                                                libc::c_ulong));
    pollfds[0 as libc::c_int as usize].fd = irc_mainport_bind();
    if pollfds[0 as libc::c_int as usize].fd < 0 as libc::c_int {
        return 1 as libc::c_int
    }
    pollfds[0 as libc::c_int as usize].events =
        0x1 as libc::c_int as libc::c_short;
    maxfds = 1 as libc::c_int;
    context = lws_create_context(&mut info);
    if context.is_null() {
        logg(((1 as libc::c_int) << 0 as libc::c_int) as libc::c_short,
             b"lws init failed\n\x00" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int
    }
    while n >= 0 as libc::c_int && interrupted == 0 {
        let mut ret: libc::c_int = 0;
        ret =
            poll(pollfds.as_mut_ptr(), maxfds as nfds_t,
                 10000 as libc::c_int);
        if ret > 0 as libc::c_int &&
               pollfds[0 as libc::c_int as usize].revents as libc::c_int ==
                   0x1 as libc::c_int {
            let mut cl_addr: sockaddr_in =
                sockaddr_in{sin_family: 0,
                            sin_port: 0,
                            sin_addr: in_addr{s_addr: 0,},
                            sin_zero: [0; 8],};
            let mut cl_len: socklen_t = 0;
            let mut newfd: libc::c_int =
                accept(pollfds[0 as libc::c_int as usize].fd,
                       __SOCKADDR_ARG{__sockaddr__:
                                          &mut cl_addr as *mut sockaddr_in as
                                              *mut sockaddr,},
                       &mut cl_len as *mut socklen_t);
            logg(((1 as libc::c_int) << 1 as libc::c_int) as libc::c_short,
                 b"spawning new fd %d\n\x00" as *const u8 as
                     *const libc::c_char, newfd);
            pollfds[maxfds as usize].fd = newfd;
            pollfds[maxfds as usize].events =
                (0x1 as libc::c_int | 0x20 as libc::c_int) as libc::c_short;
            pollfds[maxfds as usize].revents =
                0 as libc::c_int as libc::c_short;
            let mut sess: *mut t_sess = sess_new();
            (*sess).poll =
                &mut *pollfds.as_mut_ptr().offset(maxfds as isize) as
                    *mut pollfd;
            (*sess).irc_fd = newfd;
            sessions[maxfds as usize] = sess;
            maxfds += 1;
            ret -= 1
        }
        let mut current_block_103: u64;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < maxfds && ret > 0 as libc::c_int {
            let mut s: *mut t_sess = sessions[i as usize];
            if !(pollfds[i as usize].revents as libc::c_int ==
                     0 as libc::c_int) {
                ret -= 1;
                if !s.is_null() &&
                       pollfds[i as usize].revents as libc::c_int &
                           (0x20 as libc::c_int | 0x8 as libc::c_int) != 0 {
                    sess__close(s);
                    sess__free(s);
                } else {
                    if !s.is_null() && (*s).irc_fd == pollfds[i as usize].fd {
                        logg(((1 as libc::c_int) << 4 as libc::c_int) as
                                 libc::c_short,
                             b"gotcha [%d] %d -> %d\n\x00" as *const u8 as
                                 *const libc::c_char, pollfds[i as usize].fd,
                             pollfds[i as usize].events as libc::c_int,
                             pollfds[i as usize].revents as libc::c_int);
                        if pollfds[i as usize].revents as libc::c_int &
                               0x1 as libc::c_int != 0 {
                            let mut r: libc::c_int =
                                recv(pollfds[i as usize].fd,
                                     (*s).irc_buff.as_mut_ptr().offset((*s).irc_buff_head
                                                                           as
                                                                           isize)
                                         as *mut libc::c_void,
                                     (::std::mem::size_of::<[libc::c_char; 513]>()
                                          as libc::c_ulong as
                                          libc::c_ulonglong).wrapping_sub((*s).irc_buff_head
                                                                              as
                                                                              libc::c_ulonglong)
                                         as size_t, 0 as libc::c_int) as
                                    libc::c_int;
                            logg(((1 as libc::c_int) << 4 as libc::c_int) as
                                     libc::c_short,
                                 b"IRC-RX[%d]: %.*s\n\x00" as *const u8 as
                                     *const libc::c_char,
                                 pollfds[i as usize].fd, r,
                                 (*s).irc_buff.as_mut_ptr().offset((*s).irc_buff_head
                                                                       as
                                                                       isize));
                            if r == 0 as libc::c_int ||
                                   pollfds[i as usize].revents as libc::c_int
                                       & 0x10 as libc::c_int != 0 {
                                logg(((1 as libc::c_int) << 1 as libc::c_int)
                                         as libc::c_short,
                                     b"Going to close %d+%d\n\x00" as
                                         *const u8 as *const libc::c_char,
                                     pollfds[i as usize].fd,
                                     pollfds[(*s).rc.fd_idx as usize].fd);
                                shutdown(pollfds[i as usize].fd,
                                         SHUT_RDWR as libc::c_int);
                                close(pollfds[i as usize].fd);
                                (*s).rc.finished =
                                    1 as libc::c_int as libc::c_char;
                                if !(*s).rc_poll.is_null() {
                                    lws_service_fd(context, (*s).rc_poll);
                                }
                                sess__close(s);
                                sess__free(s);
                                current_block_103 = 11777552016271000781;
                            } else {
                                if r == -(1 as libc::c_int) {
                                    perror(b"what\'s the issue?\x00" as
                                               *const u8 as
                                               *const libc::c_char);
                                    exit(1 as libc::c_int);
                                }
                                (*s).irc_buff_head += r;
                                while irc__process(s, context) == 0 { }
                                current_block_103 = 7019009297990327870;
                            }
                        } else {
                            if pollfds[i as usize].revents as libc::c_int &
                                   0x4 as libc::c_int != 0 {
                                if (*s).irc_out_buff.is_null() {
                                    pollfds[i as usize].events =
                                        (pollfds[i as usize].events as
                                             libc::c_int &
                                             !(0x4 as libc::c_int)) as
                                            libc::c_short;
                                    if (*s).state & 1 as libc::c_int != 0 {
                                        if !(*s).poll.is_null() {
                                            shutdown((*(*s).poll).fd,
                                                     SHUT_RDWR as
                                                         libc::c_int);
                                        }
                                        if !(*s).rc_poll.is_null() {
                                            shutdown((*(*s).rc_poll).fd,
                                                     SHUT_RDWR as
                                                         libc::c_int);
                                        }
                                    }
                                } else {
                                    let mut r_0: libc::c_int =
                                        send(pollfds[i as usize].fd,
                                             (*(*s).irc_out_buff).start as
                                                 *const libc::c_void,
                                             (*(*s).irc_out_buff).left as
                                                 size_t, 0 as libc::c_int) as
                                            libc::c_int;
                                    if r_0 == -(1 as libc::c_int) {
                                        perror(b"Can\'t send\n\x00" as
                                                   *const u8 as
                                                   *const libc::c_char);
                                        exit(1 as libc::c_int);
                                    }
                                    logg(((1 as libc::c_int) <<
                                              4 as libc::c_int) as
                                             libc::c_short,
                                         b"IRC-TX[%d]: %.*s\n\x00" as
                                             *const u8 as *const libc::c_char,
                                         pollfds[i as usize].fd, r_0,
                                         (*(*s).irc_out_buff).start);
                                    (*(*s).irc_out_buff).start =
                                        (*(*s).irc_out_buff).start.offset(r_0
                                                                              as
                                                                              isize);
                                    (*(*s).irc_out_buff).left -= r_0;
                                    if (*(*s).irc_out_buff).left == 0 {
                                        let mut b: *mut t_buff =
                                            (*(*s).irc_out_buff).next;
                                        buff__free((*s).irc_out_buff);
                                        (*s).irc_out_buff = b;
                                        if b.is_null() {
                                            (*s).irc_out_buff_tail =
                                                &mut (*s).irc_out_buff
                                        }
                                    }
                                }
                            } else {
                                logg(((1 as libc::c_int) << 1 as libc::c_int)
                                         as libc::c_short,
                                     b"TODO[%d]: -> %x\n\x00" as *const u8 as
                                         *const libc::c_char,
                                     pollfds[i as usize].fd,
                                     pollfds[i as usize].revents as
                                         libc::c_int);
                            }
                            current_block_103 = 7019009297990327870;
                        }
                    } else {
                        logg(((1 as libc::c_int) << 4 as libc::c_int) as
                                 libc::c_short,
                             b"gotcha-2 [%d] %d -> %d\n\x00" as *const u8 as
                                 *const libc::c_char, pollfds[i as usize].fd,
                             pollfds[i as usize].events as libc::c_int,
                             pollfds[i as usize].revents as libc::c_int);
                        let mut ret_0: libc::c_int =
                            lws_service_fd(context,
                                           &mut *pollfds.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize));
                        if ret_0 != 0 {
                            logg(((1 as libc::c_int) << 0 as libc::c_int) as
                                     libc::c_short,
                                 b"lws_service_fd(%d)... %d:\n\x00" as
                                     *const u8 as *const libc::c_char,
                                 pollfds[i as usize].fd, ret_0);
                            (*sessions[i as usize]).rc.finished =
                                1 as libc::c_int as libc::c_char;
                            (*sessions[i as usize]).state |= 1 as libc::c_int;
                            sess__add_irc_out(sessions[i as usize],
                                              buff__sprintf(b"RocketChat connection error (%d)\r\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            ret_0));
                        }
                        current_block_103 = 7019009297990327870;
                    }
                    match current_block_103 {
                        11777552016271000781 => { }
                        _ => {
                            pollfds[i as usize].revents =
                                0 as libc::c_int as libc::c_short
                        }
                    }
                }
            }
            i += 1
        }
        let mut now: time_t = time(0 as *mut time_t);
        let mut s_0: *mut t_sess = session_list;
        while !s_0.is_null() {
            if (*s_0).rc.last_ping != 0 &&
                   now - (*s_0).rc.last_ping > rc_timeout as libc::c_long &&
                   (*s_0).state & 1 as libc::c_int == 0 {
                sess__add_irc_out(s_0,
                                  buff__sprintf(b"RocketChat server didn\'t ping us in %d secs, closing\r\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                now - (*s_0).rc.last_ping));
                (*s_0).state |= 1 as libc::c_int
            }
            s_0 = (*s_0).next
        }
    }
    lws_context_destroy(context);
    logg(((1 as libc::c_int) << 1 as libc::c_int) as libc::c_short,
         b"Completed\n\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}

pub fn main() {
    let args: HashSet<String> = env::args().collect();
    process::exit(main_0(args) as i32)
}
