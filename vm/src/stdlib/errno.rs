use crate::pyobject::{ItemProtocol, PyObjectRef};
use crate::VirtualMachine;

pub fn make_module(vm: &VirtualMachine) -> PyObjectRef {
    let errorcode = vm.ctx.new_dict();
    let module = py_module!(vm, "errno", {
        "errorcode" => errorcode.clone(),
    });
    for (name, code) in ERROR_CODES {
        let name = vm.new_str((*name).to_owned());
        let code = vm.ctx.new_int(*code);
        errorcode.set_item(&code, name.clone(), vm).unwrap();
        vm.set_attr(&module, name, code).unwrap();
    }
    module
}

#[cfg(target_os = "linux")]
const ERROR_CODES: &[(&str, i32)] = &[
    ("ENODEV", libc::ENODEV),
    ("ENOCSI", libc::ENOCSI),
    ("EHOSTUNREACH", libc::EHOSTUNREACH),
    ("ENOMSG", libc::ENOMSG),
    ("EUCLEAN", libc::EUCLEAN),
    ("EL2NSYNC", libc::EL2NSYNC),
    ("EL2HLT", libc::EL2HLT),
    ("ENODATA", libc::ENODATA),
    ("ENOTBLK", libc::ENOTBLK),
    ("ENOSYS", libc::ENOSYS),
    ("EPIPE", libc::EPIPE),
    ("EINVAL", libc::EINVAL),
    ("EOVERFLOW", libc::EOVERFLOW),
    ("EADV", libc::EADV),
    ("EINTR", libc::EINTR),
    ("EUSERS", libc::EUSERS),
    ("ENOTEMPTY", libc::ENOTEMPTY),
    ("ENOBUFS", libc::ENOBUFS),
    ("EPROTO", libc::EPROTO),
    ("EREMOTE", libc::EREMOTE),
    ("ENAVAIL", libc::ENAVAIL),
    ("ECHILD", libc::ECHILD),
    ("ELOOP", libc::ELOOP),
    ("EXDEV", libc::EXDEV),
    ("E2BIG", libc::E2BIG),
    ("ESRCH", libc::ESRCH),
    ("EMSGSIZE", libc::EMSGSIZE),
    ("EAFNOSUPPORT", libc::EAFNOSUPPORT),
    ("EBADR", libc::EBADR),
    ("EHOSTDOWN", libc::EHOSTDOWN),
    ("EPFNOSUPPORT", libc::EPFNOSUPPORT),
    ("ENOPROTOOPT", libc::ENOPROTOOPT),
    ("EBUSY", libc::EBUSY),
    ("EAGAIN", libc::EAGAIN),
    ("EBADFD", libc::EBADFD),
    ("EDOTDOT", libc::EDOTDOT),
    ("EISCONN", libc::EISCONN),
    ("ENOANO", libc::ENOANO),
    ("ESHUTDOWN", libc::ESHUTDOWN),
    ("ECHRNG", libc::ECHRNG),
    ("ELIBBAD", libc::ELIBBAD),
    ("ENONET", libc::ENONET),
    ("EBADE", libc::EBADE),
    ("EBADF", libc::EBADF),
    ("EMULTIHOP", libc::EMULTIHOP),
    ("EIO", libc::EIO),
    ("EUNATCH", libc::EUNATCH),
    ("EPROTOTYPE", libc::EPROTOTYPE),
    ("ENOSPC", libc::ENOSPC),
    ("ENOEXEC", libc::ENOEXEC),
    ("EALREADY", libc::EALREADY),
    ("ENETDOWN", libc::ENETDOWN),
    ("ENOTNAM", libc::ENOTNAM),
    ("EACCES", libc::EACCES),
    ("ELNRNG", libc::ELNRNG),
    ("EILSEQ", libc::EILSEQ),
    ("ENOTDIR", libc::ENOTDIR),
    ("ENOTUNIQ", libc::ENOTUNIQ),
    ("EPERM", libc::EPERM),
    ("EDOM", libc::EDOM),
    ("EXFULL", libc::EXFULL),
    ("ECONNREFUSED", libc::ECONNREFUSED),
    ("EISDIR", libc::EISDIR),
    ("EPROTONOSUPPORT", libc::EPROTONOSUPPORT),
    ("EROFS", libc::EROFS),
    ("EADDRNOTAVAIL", libc::EADDRNOTAVAIL),
    ("EIDRM", libc::EIDRM),
    ("ECOMM", libc::ECOMM),
    ("ESRMNT", libc::ESRMNT),
    ("EREMOTEIO", libc::EREMOTEIO),
    ("EL3RST", libc::EL3RST),
    ("EBADMSG", libc::EBADMSG),
    ("ENFILE", libc::ENFILE),
    ("ELIBMAX", libc::ELIBMAX),
    ("ESPIPE", libc::ESPIPE),
    ("ENOLINK", libc::ENOLINK),
    ("ENETRESET", libc::ENETRESET),
    ("ETIMEDOUT", libc::ETIMEDOUT),
    ("ENOENT", libc::ENOENT),
    ("EEXIST", libc::EEXIST),
    ("EDQUOT", libc::EDQUOT),
    ("ENOSTR", libc::ENOSTR),
    ("EBADSLT", libc::EBADSLT),
    ("EBADRQC", libc::EBADRQC),
    ("ELIBACC", libc::ELIBACC),
    ("EFAULT", libc::EFAULT),
    ("EFBIG", libc::EFBIG),
    ("EDEADLOCK", libc::EDEADLOCK),
    ("ENOTCONN", libc::ENOTCONN),
    ("EDESTADDRREQ", libc::EDESTADDRREQ),
    ("ELIBSCN", libc::ELIBSCN),
    ("ENOLCK", libc::ENOLCK),
    ("EISNAM", libc::EISNAM),
    ("ECONNABORTED", libc::ECONNABORTED),
    ("ENETUNREACH", libc::ENETUNREACH),
    ("ESTALE", libc::ESTALE),
    ("ENOSR", libc::ENOSR),
    ("ENOMEM", libc::ENOMEM),
    ("ENOTSOCK", libc::ENOTSOCK),
    ("ESTRPIPE", libc::ESTRPIPE),
    ("EMLINK", libc::EMLINK),
    ("ERANGE", libc::ERANGE),
    ("ELIBEXEC", libc::ELIBEXEC),
    ("EL3HLT", libc::EL3HLT),
    ("ECONNRESET", libc::ECONNRESET),
    ("EADDRINUSE", libc::EADDRINUSE),
    ("ENOTSUP", libc::ENOTSUP),
    ("EREMCHG", libc::EREMCHG),
    ("ENAMETOOLONG", libc::ENAMETOOLONG),
    ("ENOTTY", libc::ENOTTY),
    ("ERESTART", libc::ERESTART),
    ("ESOCKTNOSUPPORT", libc::ESOCKTNOSUPPORT),
    ("ETIME", libc::ETIME),
    ("EBFONT", libc::EBFONT),
    ("ETOOMANYREFS", libc::ETOOMANYREFS),
    ("EMFILE", libc::EMFILE),
    ("ETXTBSY", libc::ETXTBSY),
    ("EINPROGRESS", libc::EINPROGRESS),
    ("ENXIO", libc::ENXIO),
    ("ENOPKG", libc::ENOPKG),
    ("ENOMEDIUM", libc::ENOMEDIUM),
    ("EMEDIUMTYPE", libc::EMEDIUMTYPE),
    ("ECANCELED", libc::ECANCELED),
    ("ENOKEY", libc::ENOKEY),
    ("EKEYEXPIRED", libc::EKEYEXPIRED),
    ("EKEYREVOKED", libc::EKEYREVOKED),
    ("EKEYREJECTED", libc::EKEYREJECTED),
    ("EOWNERDEAD", libc::EOWNERDEAD),
    ("ENOTRECOVERABLE", libc::ENOTRECOVERABLE),
    ("ERFKILL", libc::ERFKILL),
];

#[cfg(all(unix, not(target_os = "linux")))]
const ERROR_CODES: &[(&str, i32)] = &[
    ("ENODEV", libc::ENODEV),
    ("EHOSTUNREACH", libc::EHOSTUNREACH),
    ("ENOMSG", libc::ENOMSG),
    ("ENODATA", libc::ENODATA),
    ("ENOTBLK", libc::ENOTBLK),
    ("ENOSYS", libc::ENOSYS),
    ("EPIPE", libc::EPIPE),
    ("EINVAL", libc::EINVAL),
    ("EOVERFLOW", libc::EOVERFLOW),
    ("EINTR", libc::EINTR),
    ("EUSERS", libc::EUSERS),
    ("ENOTEMPTY", libc::ENOTEMPTY),
    ("ENOBUFS", libc::ENOBUFS),
    ("EPROTO", libc::EPROTO),
    ("EREMOTE", libc::EREMOTE),
    ("ECHILD", libc::ECHILD),
    ("ELOOP", libc::ELOOP),
    ("EXDEV", libc::EXDEV),
    ("E2BIG", libc::E2BIG),
    ("ESRCH", libc::ESRCH),
    ("EMSGSIZE", libc::EMSGSIZE),
    ("EAFNOSUPPORT", libc::EAFNOSUPPORT),
    ("EHOSTDOWN", libc::EHOSTDOWN),
    ("EPFNOSUPPORT", libc::EPFNOSUPPORT),
    ("ENOPROTOOPT", libc::ENOPROTOOPT),
    ("EBUSY", libc::EBUSY),
    ("EAGAIN", libc::EAGAIN),
    ("EISCONN", libc::EISCONN),
    ("ESHUTDOWN", libc::ESHUTDOWN),
    ("EBADF", libc::EBADF),
    ("EMULTIHOP", libc::EMULTIHOP),
    ("EIO", libc::EIO),
    ("EPROTOTYPE", libc::EPROTOTYPE),
    ("ENOSPC", libc::ENOSPC),
    ("ENOEXEC", libc::ENOEXEC),
    ("EALREADY", libc::EALREADY),
    ("ENETDOWN", libc::ENETDOWN),
    ("EACCES", libc::EACCES),
    ("EILSEQ", libc::EILSEQ),
    ("ENOTDIR", libc::ENOTDIR),
    ("EPERM", libc::EPERM),
    ("EDOM", libc::EDOM),
    ("ECONNREFUSED", libc::ECONNREFUSED),
    ("EISDIR", libc::EISDIR),
    ("EPROTONOSUPPORT", libc::EPROTONOSUPPORT),
    ("EROFS", libc::EROFS),
    ("EADDRNOTAVAIL", libc::EADDRNOTAVAIL),
    ("EIDRM", libc::EIDRM),
    ("EBADMSG", libc::EBADMSG),
    ("ENFILE", libc::ENFILE),
    ("ESPIPE", libc::ESPIPE),
    ("ENOLINK", libc::ENOLINK),
    ("ENETRESET", libc::ENETRESET),
    ("ETIMEDOUT", libc::ETIMEDOUT),
    ("ENOENT", libc::ENOENT),
    ("EEXIST", libc::EEXIST),
    ("EDQUOT", libc::EDQUOT),
    ("ENOSTR", libc::ENOSTR),
    ("EFAULT", libc::EFAULT),
    ("EFBIG", libc::EFBIG),
    ("ENOTCONN", libc::ENOTCONN),
    ("EDESTADDRREQ", libc::EDESTADDRREQ),
    ("ENOLCK", libc::ENOLCK),
    ("ECONNABORTED", libc::ECONNABORTED),
    ("ENETUNREACH", libc::ENETUNREACH),
    ("ESTALE", libc::ESTALE),
    ("ENOSR", libc::ENOSR),
    ("ENOMEM", libc::ENOMEM),
    ("ENOTSOCK", libc::ENOTSOCK),
    ("EMLINK", libc::EMLINK),
    ("ERANGE", libc::ERANGE),
    ("ECONNRESET", libc::ECONNRESET),
    ("EADDRINUSE", libc::EADDRINUSE),
    #[cfg(not(target_os = "redox"))]
    ("ENOTSUP", libc::ENOTSUP),
    ("ENAMETOOLONG", libc::ENAMETOOLONG),
    ("ENOTTY", libc::ENOTTY),
    ("ESOCKTNOSUPPORT", libc::ESOCKTNOSUPPORT),
    ("ETIME", libc::ETIME),
    ("ETOOMANYREFS", libc::ETOOMANYREFS),
    ("EMFILE", libc::EMFILE),
    ("ETXTBSY", libc::ETXTBSY),
    ("EINPROGRESS", libc::EINPROGRESS),
    ("ENXIO", libc::ENXIO),
    ("ECANCELED", libc::ECANCELED),
    ("EOWNERDEAD", libc::EOWNERDEAD),
    ("ENOTRECOVERABLE", libc::ENOTRECOVERABLE),
];

#[cfg(windows)]
const ERROR_CODES: &[(&str, i32)] = &[
    ("ENODEV", 19),
    ("WSAEHOSTUNREACH", 10065),
    ("ENOMSG", 122),
    ("ENODATA", 120),
    ("ENOSYS", 40),
    ("EPIPE", 32),
    ("EINVAL", 22),
    ("EOVERFLOW", 132),
    ("EINTR", 4),
    ("WSAEUSERS", 10068),
    ("ENOTEMPTY", 41),
    ("WSAENOBUFS", 10055),
    ("EPROTO", 134),
    ("WSAEREMOTE", 10071),
    ("ECHILD", 10),
    ("WSAELOOP", 10062),
    ("EXDEV", 18),
    ("E2BIG", 7),
    ("ESRCH", 3),
    ("WSAEMSGSIZE", 10040),
    ("WSAEAFNOSUPPORT", 10047),
    ("WSAEHOSTDOWN", 10064),
    ("WSAEPFNOSUPPORT", 10046),
    ("WSAENOPROTOOPT", 10042),
    ("EBUSY", 16),
    ("WSAEWOULDBLOCK", 10035),
    ("WSAEISCONN", 10056),
    ("WSAESHUTDOWN", 10058),
    ("EBADF", 9),
    ("EIO", 5),
    ("WSAEPROTOTYPE", 10041),
    ("ENOSPC", 28),
    ("ENOEXEC", 8),
    ("WSAEALREADY", 10037),
    ("WSAENETDOWN", 10050),
    ("EACCES", 13),
    ("EILSEQ", 42),
    ("ENOTDIR", 20),
    ("EPERM", 1),
    ("EDOM", 33),
    ("WSAECONNREFUSED", 10061),
    ("EISDIR", 21),
    ("WSAEPROTONOSUPPORT", 10043),
    ("EROFS", 30),
    ("WSAEADDRNOTAVAIL", 10049),
    ("EIDRM", 111),
    ("EBADMSG", 104),
    ("ENFILE", 23),
    ("ESPIPE", 29),
    ("ENOLINK", 121),
    ("WSAENETRESET", 10052),
    ("WSAETIMEDOUT", 10060),
    ("ENOENT", 2),
    ("EEXIST", 17),
    ("WSAEDQUOT", 10069),
    ("ENOSTR", 125),
    ("EFAULT", 14),
    ("EFBIG", 27),
    ("EDEADLOCK", 36),
    ("WSAENOTCONN", 10057),
    ("WSAEDESTADDRREQ", 10039),
    ("ENOLCK", 39),
    ("WSAECONNABORTED", 10053),
    ("WSAENETUNREACH", 10051),
    ("WSAESTALE", 10070),
    ("ENOSR", 124),
    ("ENOMEM", 12),
    ("WSAENOTSOCK", 10038),
    ("EMLINK", 31),
    ("ERANGE", 34),
    ("WSAECONNRESET", 10054),
    ("WSAEADDRINUSE", 10048),
    ("WSAEOPNOTSUPP", 10045),
    ("EAGAIN", 11),
    ("ENAMETOOLONG", 38),
    ("ENOTTY", 25),
    ("WSAESOCKTNOSUPPORT", 10044),
    ("ETIME", 137),
    ("WSAETOOMANYREFS", 10059),
    ("EMFILE", 24),
    ("ETXTBSY", 139),
    ("WSAEINPROGRESS", 10036),
    ("ENXIO", 6),
    ("WSAEMFILE", 10024),
    ("WSAVERNOTSUPPORTED", 10092),
    ("WSAEPROCLIM", 10067),
    ("WSAEFAULT", 10014),
    ("WSANOTINITIALISED", 10093),
    ("WSAENAMETOOLONG", 10063),
    ("WSAENOTEMPTY", 10066),
    ("WSAEACCES", 10013),
    ("WSABASEERR", 10000),
    ("WSAEBADF", 10009),
    ("WSAEDISCON", 10101),
    ("WSAEINTR", 10004),
    ("WSASYSNOTREADY", 10091),
    ("WSAEINVAL", 10022),
    ("ECANCELED", 105),
    ("EOWNERDEAD", 133),
    ("ENOTRECOVERABLE", 127),
    ("ENOTSUP", 129),
];

#[cfg(not(any(unix, windows)))]
const ERROR_CODES: &[(&str, i32)] = &[];
