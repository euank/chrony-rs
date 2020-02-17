#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2002
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 * 
 **********************************************************************

  =======================================================================

  Header file for memory functions
  */
    /* Wrappers checking for errors */
    #[no_mangle]
    fn Malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn Realloc2(ptr: *mut libc::c_void, nmemb: size_t, size: size_t)
     -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2014
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 * 
 **********************************************************************

  =======================================================================

  Functions implementing an array with automatic memory allocation.

  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARR_Instance_Record {
    pub data: *mut libc::c_void,
    pub elem_size: libc::c_uint,
    pub used: libc::c_uint,
    pub allocated: libc::c_uint,
}
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2014
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 * 
 **********************************************************************

  =======================================================================

  Header file for array functions.
  */
pub type ARR_Instance = *mut ARR_Instance_Record;
/* Create a new array with given element size */
#[no_mangle]
pub unsafe extern "C" fn ARR_CreateInstance(mut elem_size: libc::c_uint)
 -> ARR_Instance {
    let mut array: ARR_Instance = 0 as *mut ARR_Instance_Record;
    if elem_size > 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"elem_size > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"array.c\x00" as *const u8 as *const libc::c_char,
                      47 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"ARR_Instance ARR_CreateInstance(unsigned int)\x00")).as_ptr());
    }
    array =
        Malloc(::std::mem::size_of::<ARR_Instance_Record>() as libc::c_ulong)
            as *mut ARR_Instance_Record;
    (*array).data = 0 as *mut libc::c_void;
    (*array).elem_size = elem_size;
    (*array).used = 0 as libc::c_int as libc::c_uint;
    (*array).allocated = 0 as libc::c_int as libc::c_uint;
    return array;
}
/* Destroy the array */
#[no_mangle]
pub unsafe extern "C" fn ARR_DestroyInstance(mut array: ARR_Instance) {
    free((*array).data);
    free(array as *mut libc::c_void);
}
unsafe extern "C" fn realloc_array(mut array: ARR_Instance,
                                   mut min_size: libc::c_uint) {
    if min_size <= (2 as libc::c_int as libc::c_uint).wrapping_mul(min_size) {
    } else {
        __assert_fail(b"min_size <= 2 * min_size\x00" as *const u8 as
                          *const libc::c_char,
                      b"array.c\x00" as *const u8 as *const libc::c_char,
                      69 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void realloc_array(ARR_Instance, unsigned int)\x00")).as_ptr());
    }
    if (*array).allocated >= min_size &&
           (*array).allocated <=
               (2 as libc::c_int as libc::c_uint).wrapping_mul(min_size) {
        return
    }
    if (*array).allocated < min_size {
        while (*array).allocated < min_size {
            (*array).allocated =
                if (*array).allocated != 0 {
                    (2 as libc::c_int as
                         libc::c_uint).wrapping_mul((*array).allocated)
                } else { 1 as libc::c_int as libc::c_uint }
        }
    } else { (*array).allocated = min_size }
    (*array).data =
        Realloc2((*array).data, (*array).allocated as size_t,
                 (*array).elem_size as size_t);
}
/* Return pointer to a new element added to the end of the array */
#[no_mangle]
pub unsafe extern "C" fn ARR_GetNewElement(mut array: ARR_Instance)
 -> *mut libc::c_void {
    (*array).used = (*array).used.wrapping_add(1);
    realloc_array(array, (*array).used);
    return ARR_GetElement(array,
                          (*array).used.wrapping_sub(1 as libc::c_int as
                                                         libc::c_uint));
}
/* Return element with given index */
#[no_mangle]
pub unsafe extern "C" fn ARR_GetElement(mut array: ARR_Instance,
                                        mut index: libc::c_uint)
 -> *mut libc::c_void {
    if index < (*array).used {
    } else {
        __assert_fail(b"index < array->used\x00" as *const u8 as
                          *const libc::c_char,
                      b"array.c\x00" as *const u8 as *const libc::c_char,
                      94 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"void *ARR_GetElement(ARR_Instance, unsigned int)\x00")).as_ptr());
    }
    return ((*array).data as
                *mut libc::c_char).offset((index as
                                               size_t).wrapping_mul((*array).elem_size
                                                                        as
                                                                        libc::c_ulong)
                                              as isize) as *mut libc::c_void;
}
/* Return pointer to the internal array of elements */
#[no_mangle]
pub unsafe extern "C" fn ARR_GetElements(mut array: ARR_Instance)
 -> *mut libc::c_void {
    /* Return a non-NULL pointer when the array has zero size */
    if (*array).data.is_null() {
        if (*array).used == 0 {
        } else {
            __assert_fail(b"!array->used\x00" as *const u8 as
                              *const libc::c_char,
                          b"array.c\x00" as *const u8 as *const libc::c_char,
                          103 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 36],
                                                    &[libc::c_char; 36]>(b"void *ARR_GetElements(ARR_Instance)\x00")).as_ptr());
        }
        return array as *mut libc::c_void
    }
    return (*array).data;
}
/* Add a new element to the end of the array */
#[no_mangle]
pub unsafe extern "C" fn ARR_AppendElement(mut array: ARR_Instance,
                                           mut element: *mut libc::c_void) {
    let mut e: *mut libc::c_void = 0 as *mut libc::c_void;
    e = ARR_GetNewElement(array);
    memcpy(e, element, (*array).elem_size as libc::c_ulong);
}
/* Set the size of the array */
#[no_mangle]
pub unsafe extern "C" fn ARR_SetSize(mut array: ARR_Instance,
                                     mut size: libc::c_uint) {
    realloc_array(array, size);
    (*array).used = size;
}
/* Return current size of the array */
#[no_mangle]
pub unsafe extern "C" fn ARR_GetSize(mut array: ARR_Instance)
 -> libc::c_uint {
    return (*array).used;
}
