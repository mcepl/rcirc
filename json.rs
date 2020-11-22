#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           register_tool)]
extern "C" {
    pub type json_object;
    pub type json_object_0;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jc_stack {
    pub o: *mut json_object,
    pub isarray: libc::c_int,
    pub firstchild: *mut jc_stack,
    pub parent: *mut jc_stack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jc_stack_0 {
    pub o: *mut json_object_0,
    pub isarray: libc::c_int,
    pub name: *mut libc::c_char,
    pub firstchild: *mut jc_stack_0,
    pub parent: *mut jc_stack_0,
}
#[no_mangle]
pub unsafe extern "C" fn json_read(mut data: *mut libc::c_void,
                                   mut object: *mut json_object,
                                   mut fmt: *const libc::c_char,
                                   mut args: ...) -> libc::c_int {
    let mut current_block: u64;
    let mut args_0: ::std::ffi::VaListImpl;
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    let mut z: libc::c_char = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut mode: libc::c_int = 0 as libc::c_int;
    let mut objname: [libc::c_char; 32] = [0; 32];
    let mut on_i: libc::c_int = 0 as libc::c_int;
    let mut fresh0 =
        ::std::vec::from_elem(0,
                              ::std::mem::size_of::<jc_stack>() as
                                  libc::c_ulong as usize);
    let mut root: *mut jc_stack = fresh0.as_mut_ptr() as *mut jc_stack;
    let mut top: *mut jc_stack = root;
    let mut n: *mut jc_stack = 0 as *mut jc_stack;
    (*top).parent = 0 as *mut jc_stack;
    (*top).o = object;
    let mut curr: *mut json_object = 0 as *mut json_object;
    args_0 = args.clone();
    c = fmt;
    loop  {
        z = *c;
        if !(z != 0) { current_block = 900943123863005455; break ; }
        if mode == 5 as libc::c_int {
            mode = 0 as libc::c_int;
            args_0.as_va_list().arg::<*mut libc::c_void>();
        } else if mode == 3 as libc::c_int {
            match z as libc::c_int {
                111 => { let mut v: *mut json_object = curr; cnt += 1 }
                98 => {
                    let mut v_0: libc::c_int = json_object_get_boolean(curr);
                    *args_0.as_va_list().arg::<*mut libc::c_int>() = v_0;
                    cnt += 1
                }
                100 => {
                    let mut v_1: libc::c_int = json_object_get_int(curr);
                    *args_0.as_va_list().arg::<*mut libc::c_int>() = v_1;
                    cnt += 1
                }
                115 => {
                    let mut v_2: *const libc::c_char =
                        json_object_get_string(curr) as *const libc::c_char;
                    let ref mut fresh1 =
                        *args_0.as_va_list().arg::<*mut *const libc::c_char>();
                    *fresh1 = strdup(v_2);
                    cnt += 1
                }
                _ => { current_block = 14953815020842398287; break ; }
            }
            while (*top).o == curr && !(*top).parent.is_null() {
                top = (*top).parent
            }
            curr = (*top).o;
            mode = 0 as libc::c_int
        } else if mode == 7 as libc::c_int {
            let mut r: libc::c_int = 0 as libc::c_int;
            match z as libc::c_int {
                98 => {
                    let mut v_3: libc::c_int = json_object_get_boolean(curr);
                    if args_0.as_va_list().arg::<libc::c_int>() != v_3 {
                        r = -(1 as libc::c_int)
                    }
                }
                100 => {
                    let mut v_4: libc::c_int = json_object_get_int(curr);
                    if args_0.as_va_list().arg::<libc::c_int>() != v_4 {
                        r = -(1 as libc::c_int)
                    }
                }
                115 => {
                    let mut v_5: *const libc::c_char =
                        json_object_get_string(curr) as *const libc::c_char;
                    if strcmp(args_0.as_va_list().arg::<*const libc::c_char>(),
                              v_5) != 0 {
                        r = -(1 as libc::c_int)
                    }
                }
                _ => {
                    fprintf(stderr,
                            b"Wrong usage\n\x00" as *const u8 as
                                *const libc::c_char);
                    current_block = 14953815020842398287;
                    break ;
                }
            }
            if r == -(1 as libc::c_int) {
                current_block = 14953815020842398287;
                break ;
            }
            while (*top).o == curr && !(*top).parent.is_null() {
                top = (*top).parent
            }
            curr = (*top).o;
            mode = 0 as libc::c_int
        } else {
            match z as libc::c_int {
                91 => {
                    current_block = 3388591261344650143;
                    match current_block {
                        16778818950710914692 => {
                            if mode == 4 as libc::c_int {
                                mode = 5 as libc::c_int
                            } else if mode == 6 as libc::c_int {
                                mode = 7 as libc::c_int
                            } else { mode = 3 as libc::c_int }
                        }
                        506956678842497330 => {
                            while (*top).o == curr && !(*top).parent.is_null()
                                  {
                                top = (*top).parent
                            }
                            curr = (*top).o
                        }
                        16288987300638808654 => {
                            let mut fresh3 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh3.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        413570157924672491 => {
                            top = (*top).parent;
                            curr = (*top).o
                        }
                        3388591261344650143 => {
                            let mut fresh2 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh2.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4234497564138323890 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 14953815020842398287;
                                break ;
                            }
                            let fresh5 = on_i;
                            on_i = on_i + 1;
                            objname[fresh5 as usize] = z
                        }
                        _ => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            mode =
                                if z as libc::c_int == '=' as i32 {
                                    6 as libc::c_int
                                } else { 1 as libc::c_int };
                            on_i = 0 as libc::c_int;
                            let mut fresh4 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh4.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if (*top).isarray != 0 {
                                (*n).o =
                                    json_object_array_get_idx(curr,
                                                              (*top).isarray -
                                                                  1 as
                                                                      libc::c_int)
                                        as *mut json_object;
                                (*top).isarray += 1
                            } else {
                                (*n).o =
                                    json_object_object_get(curr,
                                                           objname.as_mut_ptr())
                                        as *mut json_object
                            }
                            if (*n).o.is_null() {
                                if !(z as libc::c_int == '?' as i32) {
                                    current_block = 14953815020842398287;
                                    break ;
                                }
                                mode = 4 as libc::c_int
                            } else {
                                if (*top).firstchild.is_null() {
                                    (*top).firstchild = n
                                }
                                (*n).isarray = 0 as libc::c_int;
                                top = n;
                                curr = (*n).o
                            }
                        }
                    }
                }
                93 => {
                    current_block = 413570157924672491;
                    match current_block {
                        16778818950710914692 => {
                            if mode == 4 as libc::c_int {
                                mode = 5 as libc::c_int
                            } else if mode == 6 as libc::c_int {
                                mode = 7 as libc::c_int
                            } else { mode = 3 as libc::c_int }
                        }
                        506956678842497330 => {
                            while (*top).o == curr && !(*top).parent.is_null()
                                  {
                                top = (*top).parent
                            }
                            curr = (*top).o
                        }
                        16288987300638808654 => {
                            let mut fresh3 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh3.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        413570157924672491 => {
                            top = (*top).parent;
                            curr = (*top).o
                        }
                        3388591261344650143 => {
                            let mut fresh2 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh2.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4234497564138323890 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 14953815020842398287;
                                break ;
                            }
                            let fresh5 = on_i;
                            on_i = on_i + 1;
                            objname[fresh5 as usize] = z
                        }
                        _ => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            mode =
                                if z as libc::c_int == '=' as i32 {
                                    6 as libc::c_int
                                } else { 1 as libc::c_int };
                            on_i = 0 as libc::c_int;
                            let mut fresh4 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh4.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if (*top).isarray != 0 {
                                (*n).o =
                                    json_object_array_get_idx(curr,
                                                              (*top).isarray -
                                                                  1 as
                                                                      libc::c_int)
                                        as *mut json_object;
                                (*top).isarray += 1
                            } else {
                                (*n).o =
                                    json_object_object_get(curr,
                                                           objname.as_mut_ptr())
                                        as *mut json_object
                            }
                            if (*n).o.is_null() {
                                if !(z as libc::c_int == '?' as i32) {
                                    current_block = 14953815020842398287;
                                    break ;
                                }
                                mode = 4 as libc::c_int
                            } else {
                                if (*top).firstchild.is_null() {
                                    (*top).firstchild = n
                                }
                                (*n).isarray = 0 as libc::c_int;
                                top = n;
                                curr = (*n).o
                            }
                        }
                    }
                }
                123 => {
                    current_block = 16288987300638808654;
                    match current_block {
                        16778818950710914692 => {
                            if mode == 4 as libc::c_int {
                                mode = 5 as libc::c_int
                            } else if mode == 6 as libc::c_int {
                                mode = 7 as libc::c_int
                            } else { mode = 3 as libc::c_int }
                        }
                        506956678842497330 => {
                            while (*top).o == curr && !(*top).parent.is_null()
                                  {
                                top = (*top).parent
                            }
                            curr = (*top).o
                        }
                        16288987300638808654 => {
                            let mut fresh3 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh3.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        413570157924672491 => {
                            top = (*top).parent;
                            curr = (*top).o
                        }
                        3388591261344650143 => {
                            let mut fresh2 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh2.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4234497564138323890 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 14953815020842398287;
                                break ;
                            }
                            let fresh5 = on_i;
                            on_i = on_i + 1;
                            objname[fresh5 as usize] = z
                        }
                        _ => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            mode =
                                if z as libc::c_int == '=' as i32 {
                                    6 as libc::c_int
                                } else { 1 as libc::c_int };
                            on_i = 0 as libc::c_int;
                            let mut fresh4 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh4.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if (*top).isarray != 0 {
                                (*n).o =
                                    json_object_array_get_idx(curr,
                                                              (*top).isarray -
                                                                  1 as
                                                                      libc::c_int)
                                        as *mut json_object;
                                (*top).isarray += 1
                            } else {
                                (*n).o =
                                    json_object_object_get(curr,
                                                           objname.as_mut_ptr())
                                        as *mut json_object
                            }
                            if (*n).o.is_null() {
                                if !(z as libc::c_int == '?' as i32) {
                                    current_block = 14953815020842398287;
                                    break ;
                                }
                                mode = 4 as libc::c_int
                            } else {
                                if (*top).firstchild.is_null() {
                                    (*top).firstchild = n
                                }
                                (*n).isarray = 0 as libc::c_int;
                                top = n;
                                curr = (*n).o
                            }
                        }
                    }
                }
                125 => {
                    current_block = 506956678842497330;
                    match current_block {
                        16778818950710914692 => {
                            if mode == 4 as libc::c_int {
                                mode = 5 as libc::c_int
                            } else if mode == 6 as libc::c_int {
                                mode = 7 as libc::c_int
                            } else { mode = 3 as libc::c_int }
                        }
                        506956678842497330 => {
                            while (*top).o == curr && !(*top).parent.is_null()
                                  {
                                top = (*top).parent
                            }
                            curr = (*top).o
                        }
                        16288987300638808654 => {
                            let mut fresh3 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh3.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        413570157924672491 => {
                            top = (*top).parent;
                            curr = (*top).o
                        }
                        3388591261344650143 => {
                            let mut fresh2 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh2.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4234497564138323890 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 14953815020842398287;
                                break ;
                            }
                            let fresh5 = on_i;
                            on_i = on_i + 1;
                            objname[fresh5 as usize] = z
                        }
                        _ => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            mode =
                                if z as libc::c_int == '=' as i32 {
                                    6 as libc::c_int
                                } else { 1 as libc::c_int };
                            on_i = 0 as libc::c_int;
                            let mut fresh4 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh4.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if (*top).isarray != 0 {
                                (*n).o =
                                    json_object_array_get_idx(curr,
                                                              (*top).isarray -
                                                                  1 as
                                                                      libc::c_int)
                                        as *mut json_object;
                                (*top).isarray += 1
                            } else {
                                (*n).o =
                                    json_object_object_get(curr,
                                                           objname.as_mut_ptr())
                                        as *mut json_object
                            }
                            if (*n).o.is_null() {
                                if !(z as libc::c_int == '?' as i32) {
                                    current_block = 14953815020842398287;
                                    break ;
                                }
                                mode = 4 as libc::c_int
                            } else {
                                if (*top).firstchild.is_null() {
                                    (*top).firstchild = n
                                }
                                (*n).isarray = 0 as libc::c_int;
                                top = n;
                                curr = (*n).o
                            }
                        }
                    }
                }
                32 => { }
                63 | 61 | 58 => {
                    current_block = 16157800273112125286;
                    match current_block {
                        16778818950710914692 => {
                            if mode == 4 as libc::c_int {
                                mode = 5 as libc::c_int
                            } else if mode == 6 as libc::c_int {
                                mode = 7 as libc::c_int
                            } else { mode = 3 as libc::c_int }
                        }
                        506956678842497330 => {
                            while (*top).o == curr && !(*top).parent.is_null()
                                  {
                                top = (*top).parent
                            }
                            curr = (*top).o
                        }
                        16288987300638808654 => {
                            let mut fresh3 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh3.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        413570157924672491 => {
                            top = (*top).parent;
                            curr = (*top).o
                        }
                        3388591261344650143 => {
                            let mut fresh2 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh2.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4234497564138323890 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 14953815020842398287;
                                break ;
                            }
                            let fresh5 = on_i;
                            on_i = on_i + 1;
                            objname[fresh5 as usize] = z
                        }
                        _ => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            mode =
                                if z as libc::c_int == '=' as i32 {
                                    6 as libc::c_int
                                } else { 1 as libc::c_int };
                            on_i = 0 as libc::c_int;
                            let mut fresh4 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh4.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if (*top).isarray != 0 {
                                (*n).o =
                                    json_object_array_get_idx(curr,
                                                              (*top).isarray -
                                                                  1 as
                                                                      libc::c_int)
                                        as *mut json_object;
                                (*top).isarray += 1
                            } else {
                                (*n).o =
                                    json_object_object_get(curr,
                                                           objname.as_mut_ptr())
                                        as *mut json_object
                            }
                            if (*n).o.is_null() {
                                if !(z as libc::c_int == '?' as i32) {
                                    current_block = 14953815020842398287;
                                    break ;
                                }
                                mode = 4 as libc::c_int
                            } else {
                                if (*top).firstchild.is_null() {
                                    (*top).firstchild = n
                                }
                                (*n).isarray = 0 as libc::c_int;
                                top = n;
                                curr = (*n).o
                            }
                        }
                    }
                }
                37 => {
                    current_block = 16778818950710914692;
                    match current_block {
                        16778818950710914692 => {
                            if mode == 4 as libc::c_int {
                                mode = 5 as libc::c_int
                            } else if mode == 6 as libc::c_int {
                                mode = 7 as libc::c_int
                            } else { mode = 3 as libc::c_int }
                        }
                        506956678842497330 => {
                            while (*top).o == curr && !(*top).parent.is_null()
                                  {
                                top = (*top).parent
                            }
                            curr = (*top).o
                        }
                        16288987300638808654 => {
                            let mut fresh3 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh3.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        413570157924672491 => {
                            top = (*top).parent;
                            curr = (*top).o
                        }
                        3388591261344650143 => {
                            let mut fresh2 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh2.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4234497564138323890 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 14953815020842398287;
                                break ;
                            }
                            let fresh5 = on_i;
                            on_i = on_i + 1;
                            objname[fresh5 as usize] = z
                        }
                        _ => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            mode =
                                if z as libc::c_int == '=' as i32 {
                                    6 as libc::c_int
                                } else { 1 as libc::c_int };
                            on_i = 0 as libc::c_int;
                            let mut fresh4 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh4.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if (*top).isarray != 0 {
                                (*n).o =
                                    json_object_array_get_idx(curr,
                                                              (*top).isarray -
                                                                  1 as
                                                                      libc::c_int)
                                        as *mut json_object;
                                (*top).isarray += 1
                            } else {
                                (*n).o =
                                    json_object_object_get(curr,
                                                           objname.as_mut_ptr())
                                        as *mut json_object
                            }
                            if (*n).o.is_null() {
                                if !(z as libc::c_int == '?' as i32) {
                                    current_block = 14953815020842398287;
                                    break ;
                                }
                                mode = 4 as libc::c_int
                            } else {
                                if (*top).firstchild.is_null() {
                                    (*top).firstchild = n
                                }
                                (*n).isarray = 0 as libc::c_int;
                                top = n;
                                curr = (*n).o
                            }
                        }
                    }
                }
                _ => {
                    current_block = 4234497564138323890;
                    match current_block {
                        16778818950710914692 => {
                            if mode == 4 as libc::c_int {
                                mode = 5 as libc::c_int
                            } else if mode == 6 as libc::c_int {
                                mode = 7 as libc::c_int
                            } else { mode = 3 as libc::c_int }
                        }
                        506956678842497330 => {
                            while (*top).o == curr && !(*top).parent.is_null()
                                  {
                                top = (*top).parent
                            }
                            curr = (*top).o
                        }
                        16288987300638808654 => {
                            let mut fresh3 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh3.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        413570157924672491 => {
                            top = (*top).parent;
                            curr = (*top).o
                        }
                        3388591261344650143 => {
                            let mut fresh2 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh2.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if curr.is_null() { curr = object }
                            (*n).o = curr;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4234497564138323890 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 14953815020842398287;
                                break ;
                            }
                            let fresh5 = on_i;
                            on_i = on_i + 1;
                            objname[fresh5 as usize] = z
                        }
                        _ => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            mode =
                                if z as libc::c_int == '=' as i32 {
                                    6 as libc::c_int
                                } else { 1 as libc::c_int };
                            on_i = 0 as libc::c_int;
                            let mut fresh4 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh4.as_mut_ptr() as *mut jc_stack;
                            (*n).parent = top;
                            if (*top).isarray != 0 {
                                (*n).o =
                                    json_object_array_get_idx(curr,
                                                              (*top).isarray -
                                                                  1 as
                                                                      libc::c_int)
                                        as *mut json_object;
                                (*top).isarray += 1
                            } else {
                                (*n).o =
                                    json_object_object_get(curr,
                                                           objname.as_mut_ptr())
                                        as *mut json_object
                            }
                            if (*n).o.is_null() {
                                if !(z as libc::c_int == '?' as i32) {
                                    current_block = 14953815020842398287;
                                    break ;
                                }
                                mode = 4 as libc::c_int
                            } else {
                                if (*top).firstchild.is_null() {
                                    (*top).firstchild = n
                                }
                                (*n).isarray = 0 as libc::c_int;
                                top = n;
                                curr = (*n).o
                            }
                        }
                    }
                }
            }
        }
        c = c.offset(1)
    }
    match current_block {
        900943123863005455 => { return cnt }
        _ => { return -(1 as libc::c_int) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_create(mut data: *mut libc::c_void,
                                     mut fmt: *const libc::c_char,
                                     mut args: ...) -> *mut json_object_0 {
    let mut current_block: u64;
    let mut args_0: ::std::ffi::VaListImpl;
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    let mut z: libc::c_char = 0;
    let mut mode: libc::c_int = 0 as libc::c_int;
    let mut objname: [libc::c_char; 32] = [0; 32];
    let mut on_i: libc::c_int = 0 as libc::c_int;
    let mut fresh6 =
        ::std::vec::from_elem(0,
                              ::std::mem::size_of::<jc_stack_0>() as
                                  libc::c_ulong as usize);
    let mut root: *mut jc_stack_0 = fresh6.as_mut_ptr() as *mut jc_stack_0;
    let mut top: *mut jc_stack_0 = root;
    let mut n: *mut jc_stack_0 = 0 as *mut jc_stack_0;
    (*top).parent = 0 as *mut jc_stack_0;
    (*top).o = 0 as *mut json_object_0;
    args_0 = args.clone();
    c = fmt;
    loop  {
        z = *c;
        if !(z != 0) { current_block = 9925100494328262799; break ; }
        if mode == 3 as libc::c_int {
            let mut s: *mut json_object_0 = 0 as *mut json_object_0;
            match z as libc::c_int {
                98 => {
                    s =
                        json_object_new_boolean(args_0.as_va_list().arg::<libc::c_int>())
                            as *mut json_object_0
                }
                100 => {
                    s =
                        json_object_new_int(args_0.as_va_list().arg::<libc::c_int>())
                            as *mut json_object_0
                }
                115 => {
                    s =
                        json_object_new_string(args_0.as_va_list().arg::<*mut libc::c_char>())
                            as *mut json_object_0
                }
                111 => {
                    s =
                        args_0.as_va_list().arg::<*mut libc::c_void>() as
                            *mut json_object_0
                }
                _ => { current_block = 1874315696050160458; break ; }
            }
            if (*top).isarray != 0 {
                json_object_array_add((*top).o, s);
            } else { json_object_object_add((*top).o, (*top).name, s); }
            mode = 0 as libc::c_int
        } else {
            match z as libc::c_int {
                91 => {
                    current_block = 16034792509161872205;
                    match current_block {
                        2241680329088345986 => { mode = 3 as libc::c_int }
                        5145529149066033896 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        16034792509161872205 => {
                            let mut fresh7 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh7.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_array() as *mut json_object_0;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4723370291228316244 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        4510563271777997454 => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            (*top).name =
                                ({
                                     let mut __old: *const libc::c_char =
                                         objname.as_mut_ptr();
                                     let mut __len: size_t =
                                         strlen(__old).wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong);
                                     let mut fresh9 =
                                         ::std::vec::from_elem(0,
                                                               __len as
                                                                   usize);
                                     let mut __new: *mut libc::c_char =
                                         fresh9.as_mut_ptr() as
                                             *mut libc::c_char;
                                     memcpy(__new as *mut libc::c_void,
                                            __old as *const libc::c_void,
                                            __len) as *mut libc::c_char
                                 });
                            mode = 1 as libc::c_int;
                            on_i = 0 as libc::c_int
                        }
                        10983261971688379477 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 1874315696050160458;
                                break ;
                            }
                            let fresh10 = on_i;
                            on_i = on_i + 1;
                            objname[fresh10 as usize] = z
                        }
                        _ => {
                            let mut fresh8 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh8.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_object() as
                                    *mut json_object_0;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                    }
                }
                93 => {
                    current_block = 5145529149066033896;
                    match current_block {
                        2241680329088345986 => { mode = 3 as libc::c_int }
                        5145529149066033896 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        16034792509161872205 => {
                            let mut fresh7 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh7.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_array() as *mut json_object_0;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4723370291228316244 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        4510563271777997454 => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            (*top).name =
                                ({
                                     let mut __old: *const libc::c_char =
                                         objname.as_mut_ptr();
                                     let mut __len: size_t =
                                         strlen(__old).wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong);
                                     let mut fresh9 =
                                         ::std::vec::from_elem(0,
                                                               __len as
                                                                   usize);
                                     let mut __new: *mut libc::c_char =
                                         fresh9.as_mut_ptr() as
                                             *mut libc::c_char;
                                     memcpy(__new as *mut libc::c_void,
                                            __old as *const libc::c_void,
                                            __len) as *mut libc::c_char
                                 });
                            mode = 1 as libc::c_int;
                            on_i = 0 as libc::c_int
                        }
                        10983261971688379477 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 1874315696050160458;
                                break ;
                            }
                            let fresh10 = on_i;
                            on_i = on_i + 1;
                            objname[fresh10 as usize] = z
                        }
                        _ => {
                            let mut fresh8 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh8.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_object() as
                                    *mut json_object_0;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                    }
                }
                123 => {
                    current_block = 7240016366304774481;
                    match current_block {
                        2241680329088345986 => { mode = 3 as libc::c_int }
                        5145529149066033896 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        16034792509161872205 => {
                            let mut fresh7 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh7.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_array() as *mut json_object_0;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4723370291228316244 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        4510563271777997454 => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            (*top).name =
                                ({
                                     let mut __old: *const libc::c_char =
                                         objname.as_mut_ptr();
                                     let mut __len: size_t =
                                         strlen(__old).wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong);
                                     let mut fresh9 =
                                         ::std::vec::from_elem(0,
                                                               __len as
                                                                   usize);
                                     let mut __new: *mut libc::c_char =
                                         fresh9.as_mut_ptr() as
                                             *mut libc::c_char;
                                     memcpy(__new as *mut libc::c_void,
                                            __old as *const libc::c_void,
                                            __len) as *mut libc::c_char
                                 });
                            mode = 1 as libc::c_int;
                            on_i = 0 as libc::c_int
                        }
                        10983261971688379477 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 1874315696050160458;
                                break ;
                            }
                            let fresh10 = on_i;
                            on_i = on_i + 1;
                            objname[fresh10 as usize] = z
                        }
                        _ => {
                            let mut fresh8 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh8.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_object() as
                                    *mut json_object_0;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                    }
                }
                125 => {
                    current_block = 4723370291228316244;
                    match current_block {
                        2241680329088345986 => { mode = 3 as libc::c_int }
                        5145529149066033896 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        16034792509161872205 => {
                            let mut fresh7 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh7.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_array() as *mut json_object_0;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4723370291228316244 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        4510563271777997454 => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            (*top).name =
                                ({
                                     let mut __old: *const libc::c_char =
                                         objname.as_mut_ptr();
                                     let mut __len: size_t =
                                         strlen(__old).wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong);
                                     let mut fresh9 =
                                         ::std::vec::from_elem(0,
                                                               __len as
                                                                   usize);
                                     let mut __new: *mut libc::c_char =
                                         fresh9.as_mut_ptr() as
                                             *mut libc::c_char;
                                     memcpy(__new as *mut libc::c_void,
                                            __old as *const libc::c_void,
                                            __len) as *mut libc::c_char
                                 });
                            mode = 1 as libc::c_int;
                            on_i = 0 as libc::c_int
                        }
                        10983261971688379477 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 1874315696050160458;
                                break ;
                            }
                            let fresh10 = on_i;
                            on_i = on_i + 1;
                            objname[fresh10 as usize] = z
                        }
                        _ => {
                            let mut fresh8 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh8.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_object() as
                                    *mut json_object_0;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                    }
                }
                32 => { }
                58 => {
                    current_block = 4510563271777997454;
                    match current_block {
                        2241680329088345986 => { mode = 3 as libc::c_int }
                        5145529149066033896 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        16034792509161872205 => {
                            let mut fresh7 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh7.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_array() as *mut json_object_0;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4723370291228316244 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        4510563271777997454 => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            (*top).name =
                                ({
                                     let mut __old: *const libc::c_char =
                                         objname.as_mut_ptr();
                                     let mut __len: size_t =
                                         strlen(__old).wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong);
                                     let mut fresh9 =
                                         ::std::vec::from_elem(0,
                                                               __len as
                                                                   usize);
                                     let mut __new: *mut libc::c_char =
                                         fresh9.as_mut_ptr() as
                                             *mut libc::c_char;
                                     memcpy(__new as *mut libc::c_void,
                                            __old as *const libc::c_void,
                                            __len) as *mut libc::c_char
                                 });
                            mode = 1 as libc::c_int;
                            on_i = 0 as libc::c_int
                        }
                        10983261971688379477 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 1874315696050160458;
                                break ;
                            }
                            let fresh10 = on_i;
                            on_i = on_i + 1;
                            objname[fresh10 as usize] = z
                        }
                        _ => {
                            let mut fresh8 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh8.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_object() as
                                    *mut json_object_0;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                    }
                }
                37 => {
                    current_block = 2241680329088345986;
                    match current_block {
                        2241680329088345986 => { mode = 3 as libc::c_int }
                        5145529149066033896 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        16034792509161872205 => {
                            let mut fresh7 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh7.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_array() as *mut json_object_0;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4723370291228316244 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        4510563271777997454 => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            (*top).name =
                                ({
                                     let mut __old: *const libc::c_char =
                                         objname.as_mut_ptr();
                                     let mut __len: size_t =
                                         strlen(__old).wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong);
                                     let mut fresh9 =
                                         ::std::vec::from_elem(0,
                                                               __len as
                                                                   usize);
                                     let mut __new: *mut libc::c_char =
                                         fresh9.as_mut_ptr() as
                                             *mut libc::c_char;
                                     memcpy(__new as *mut libc::c_void,
                                            __old as *const libc::c_void,
                                            __len) as *mut libc::c_char
                                 });
                            mode = 1 as libc::c_int;
                            on_i = 0 as libc::c_int
                        }
                        10983261971688379477 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 1874315696050160458;
                                break ;
                            }
                            let fresh10 = on_i;
                            on_i = on_i + 1;
                            objname[fresh10 as usize] = z
                        }
                        _ => {
                            let mut fresh8 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh8.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_object() as
                                    *mut json_object_0;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                    }
                }
                _ => {
                    current_block = 10983261971688379477;
                    match current_block {
                        2241680329088345986 => { mode = 3 as libc::c_int }
                        5145529149066033896 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        16034792509161872205 => {
                            let mut fresh7 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh7.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_array() as *mut json_object_0;
                            (*n).isarray = 1 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                        4723370291228316244 => {
                            if !(*(*top).parent).o.is_null() {
                                if (*(*top).parent).isarray != 0 {
                                    json_object_array_add((*(*top).parent).o,
                                                          (*top).o);
                                } else {
                                    json_object_object_add((*(*top).parent).o,
                                                           (*(*top).parent).name,
                                                           (*top).o);
                                }
                            }
                            top = (*top).parent
                        }
                        4510563271777997454 => {
                            objname[on_i as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            (*top).name =
                                ({
                                     let mut __old: *const libc::c_char =
                                         objname.as_mut_ptr();
                                     let mut __len: size_t =
                                         strlen(__old).wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong);
                                     let mut fresh9 =
                                         ::std::vec::from_elem(0,
                                                               __len as
                                                                   usize);
                                     let mut __new: *mut libc::c_char =
                                         fresh9.as_mut_ptr() as
                                             *mut libc::c_char;
                                     memcpy(__new as *mut libc::c_void,
                                            __old as *const libc::c_void,
                                            __len) as *mut libc::c_char
                                 });
                            mode = 1 as libc::c_int;
                            on_i = 0 as libc::c_int
                        }
                        10983261971688379477 => {
                            if on_i as libc::c_ulong >=
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong {
                                current_block = 1874315696050160458;
                                break ;
                            }
                            let fresh10 = on_i;
                            on_i = on_i + 1;
                            objname[fresh10 as usize] = z
                        }
                        _ => {
                            let mut fresh8 =
                                ::std::vec::from_elem(0,
                                                      ::std::mem::size_of::<jc_stack_0>()
                                                          as libc::c_ulong as
                                                          usize);
                            n = fresh8.as_mut_ptr() as *mut jc_stack_0;
                            (*n).parent = top;
                            (*n).name = 0 as *mut libc::c_char;
                            (*n).o =
                                json_object_new_object() as
                                    *mut json_object_0;
                            (*n).isarray = 0 as libc::c_int;
                            (*top).firstchild = n;
                            top = n
                        }
                    }
                }
            }
        }
        c = c.offset(1)
    }
    match current_block {
        1874315696050160458 => { return 0 as *mut json_object_0 }
        _ => {
            while !(*top).parent.is_null() && !(*(*top).parent).o.is_null() {
                top = (*top).parent
            }
            return (*(*root).firstchild).o
        }
    };
}
