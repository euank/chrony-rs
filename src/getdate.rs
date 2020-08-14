#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, ptr_wrapping_offset_from,
           register_tool)]
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* Suppress unused-variable warnings by "using" E.  */
    /* The parser invokes alloca or malloc; define the necessary symbols.  */
    /* Include this file down here because bison inserts code above which
    may define-away `const'.  We want the prototype for get_date to have
    the same signature as the function definition does. */
    #[no_mangle]
    fn gmtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
}
pub type __time_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub const tID: yytokentype = 264;
/* size of buffer to read the date into */
/*
**  An entry in the lexical lookup table.
*/
pub type TABLE = _TABLE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _TABLE {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub value: libc::c_int,
}
/* Value type.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub Number: libc::c_int,
    pub Meridian: _MERIDIAN,
}
pub type _MERIDIAN = libc::c_uint;
pub const MER24: _MERIDIAN = 2;
pub const MERpm: _MERIDIAN = 1;
pub const MERam: _MERIDIAN = 0;
pub const tZONE: yytokentype = 273;
pub const tDAYZONE: yytokentype = 261;
pub const tAGO: yytokentype = 258;
pub const tUNUMBER: yytokentype = 271;
pub const tMINUTE_UNIT: yytokentype = 266;
pub const tSEC_UNIT: yytokentype = 269;
pub const tHOUR_UNIT: yytokentype = 263;
pub const tDAY_UNIT: yytokentype = 260;
pub const tMONTH_UNIT: yytokentype = 268;
pub const tYEAR_UNIT: yytokentype = 272;
pub const tDST: yytokentype = 262;
pub const tDAY: yytokentype = 259;
pub const tMONTH: yytokentype = 267;
/* Interpret as YYYY/MM/DD if $1 >= 1000, otherwise as MM/DD/YY.
The goal in recognizing YYYY/MM/DD is solely to support legacy
machine-generated dates like those in an RCS log listing.  If
you want portability, use the ISO 8601 format.  */
pub const tMERIDIAN: yytokentype = 265;
pub const tSNUMBER: yytokentype = 270;
/*
**  Meridian:  am, pm, or 24-hour style.
*/
pub type MERIDIAN = _MERIDIAN;
/* Enabling verbose error messages.  */
/* Debug traces.  */
/* Token type.  */
pub type yytokentype = libc::c_uint;
pub type yytype_int16 = libc::c_short;
pub type yytype_uint8 = libc::c_uchar;
pub type yytype_int8 = libc::c_schar;
/* A type that is properly aligned for any stack member.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yytype_int16,
    pub yyvs_alloc: YYSTYPE,
}
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
/*
**  Global variables.  We could get rid of most of these by using a good
**  union as the yacc stack.  (This routine was originally written before
**  yacc had the %union construct.)  Maybe someday; right now we only use
**  the %union very rarely.
*/
static mut yyInput: *const libc::c_char = 0 as *const libc::c_char;
static mut yyDayOrdinal: libc::c_int = 0;
static mut yyDayNumber: libc::c_int = 0;
static mut yyHaveDate: libc::c_int = 0;
static mut yyHaveDay: libc::c_int = 0;
static mut yyHaveRel: libc::c_int = 0;
static mut yyHaveTime: libc::c_int = 0;
static mut yyHaveZone: libc::c_int = 0;
static mut yyTimezone: libc::c_int = 0;
static mut yyDay: libc::c_int = 0;
static mut yyHour: libc::c_int = 0;
static mut yyMinutes: libc::c_int = 0;
static mut yyMonth: libc::c_int = 0;
static mut yySeconds: libc::c_int = 0;
static mut yyYear: libc::c_int = 0;
static mut yyMeridian: MERIDIAN = MERam;
static mut yyRelDay: libc::c_int = 0;
static mut yyRelHour: libc::c_int = 0;
static mut yyRelMinutes: libc::c_int = 0;
static mut yyRelMonth: libc::c_int = 0;
static mut yyRelSeconds: libc::c_int = 0;
static mut yyRelYear: libc::c_int = 0;
/* Month and day table. */
static mut MonthDayTable: [TABLE; 25] = [
    {
        let mut init = _TABLE {
            name: b"january\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"february\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"march\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"april\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"may\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"june\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"july\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"august\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"september\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"sept\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"october\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"november\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"december\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"sunday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"monday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"tuesday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"tues\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"wednesday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"wednes\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"thursday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"thur\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"thurs\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"friday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"saturday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut UnitsTable: [TABLE; 11] = [
    {
        let mut init = _TABLE {
            name: b"year\x00" as *const u8 as *const libc::c_char,
            type_0: tYEAR_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"month\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"fortnight\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY_UNIT as libc::c_int,
            value: 14 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"week\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY_UNIT as libc::c_int,
            value: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"day\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"hour\x00" as *const u8 as *const libc::c_char,
            type_0: tHOUR_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"minute\x00" as *const u8 as *const libc::c_char,
            type_0: tMINUTE_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"min\x00" as *const u8 as *const libc::c_char,
            type_0: tMINUTE_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"second\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"sec\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut OtherTable: [TABLE; 20] = [
    {
        let mut init = _TABLE {
            name: b"tomorrow\x00" as *const u8 as *const libc::c_char,
            type_0: tMINUTE_UNIT as libc::c_int,
            value: 1 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"yesterday\x00" as *const u8 as *const libc::c_char,
            type_0: tMINUTE_UNIT as libc::c_int,
            value: -(1 as libc::c_int) * 24 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"today\x00" as *const u8 as *const libc::c_char,
            type_0: tMINUTE_UNIT as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"now\x00" as *const u8 as *const libc::c_char,
            type_0: tMINUTE_UNIT as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"last\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"this\x00" as *const u8 as *const libc::c_char,
            type_0: tMINUTE_UNIT as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"next\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"first\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"third\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"fourth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"fifth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"sixth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"seventh\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"eighth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"ninth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"tenth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"eleventh\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"twelfth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"ago\x00" as *const u8 as *const libc::c_char,
            type_0: tAGO as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
/* The timezone table. */
static mut TimezoneTable: [TABLE; 51] = [
    {
        let mut init = _TABLE {
            name: b"gmt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 0 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"ut\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 0 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"utc\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 0 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"wet\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 0 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"bst\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 0 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"wat\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 1 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"at\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 2 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"ast\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 4 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"adt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 4 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"est\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 5 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"edt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 5 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"cst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 6 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"cdt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 6 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"mst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 7 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"mdt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 7 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"pst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 8 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"pdt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 8 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"yst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 9 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"ydt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 9 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"hst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 10 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"hdt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 10 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"cat\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 10 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"ahst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 10 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"nt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 11 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"idlw\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 12 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"cet\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(1 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"met\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(1 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"mewt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(1 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"mest\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(1 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"mesz\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(1 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"swt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(1 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"sst\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(1 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"fwt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(1 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"fst\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(1 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"eet\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(2 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"bt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(3 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"zp4\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(4 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"zp5\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(5 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"zp6\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(6 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"wast\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(7 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"wadt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(7 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"cct\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(8 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"jst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(9 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"east\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(10 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"eadt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(10 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"gst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(10 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"nzt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(12 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"nzst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(12 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"nzdt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(12 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"idle\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(12 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = _TABLE {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut yytranslate: [yytype_uint8; 274] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
];
/* Military timezone table. */
static mut MilitaryTable: [TABLE; 26] = [
    {
        let mut init = _TABLE {
            name: b"a\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 1 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"b\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 2 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"c\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 3 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"d\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 4 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"e\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 5 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"f\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 6 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"g\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 7 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"h\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 8 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"i\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 9 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"k\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 10 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"l\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 11 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"m\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 12 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"n\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(1 as libc::c_int) * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"o\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(2 as libc::c_int) * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"p\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(3 as libc::c_int) * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"q\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(4 as libc::c_int) * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"r\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(5 as libc::c_int) * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"s\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(6 as libc::c_int) * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"t\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(7 as libc::c_int) * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"u\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(8 as libc::c_int) * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"v\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(9 as libc::c_int) * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"w\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(10 as libc::c_int) * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"x\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(11 as libc::c_int) * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"y\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(12 as libc::c_int) * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: b"z\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 0 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = _TABLE {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
/* ARGSUSED */
unsafe extern "C" fn gd_error(mut s: *mut libc::c_char) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn ToHour(mut Hours: libc::c_int, mut Meridian: MERIDIAN) -> libc::c_int {
    match Meridian as libc::c_uint {
        2 => {
            /* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
            STATE-NUM.  */
            if Hours < 0 as libc::c_int || Hours > 23 as libc::c_int {
                return -(1 as libc::c_int);
            }
            return Hours;
        }
        0 => {
            if Hours < 1 as libc::c_int || Hours > 12 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if Hours == 12 as libc::c_int {
                Hours = 0 as libc::c_int
            }
            return Hours;
        }
        1 => {
            if Hours < 1 as libc::c_int || Hours > 12 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if Hours == 12 as libc::c_int {
                Hours = 0 as libc::c_int
            }
            return Hours + 12 as libc::c_int;
        }
        _ => {
            abort();
        }
    };
    /* NOTREACHED */
}
static mut gd_pact: [yytype_int8; 61] = [
    -(20 as libc::c_int) as yytype_int8,
    0 as libc::c_int as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(19 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(13 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    30 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    14 as libc::c_int as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    19 as libc::c_int as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    4 as libc::c_int as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(6 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    16 as libc::c_int as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    17 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    24 as libc::c_int as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    29 as libc::c_int as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    32 as libc::c_int as yytype_int8,
    -(8 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
];
static mut yydefact: [yytype_uint8; 61] = [
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
];
unsafe extern "C" fn ToYear(mut Year: libc::c_int) -> libc::c_int {
    if Year < 0 as libc::c_int {
        Year = -Year
    }
    /* XPG4 suggests that years 00-68 map to 2000-2068, and
    years 69-99 map to 1969-1999.  */
    if Year < 69 as libc::c_int {
        Year += 2000 as libc::c_int
    } else if Year < 100 as libc::c_int {
        Year += 1900 as libc::c_int
    }
    return Year;
}
static mut yypgoto: [yytype_int8; 11] = [
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(20 as libc::c_int) as yytype_int8,
    -(7 as libc::c_int) as yytype_int8,
];
static mut yydefgoto: [yytype_int8; 11] = [
    -(1 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn LookupWord(mut buff: *mut libc::c_char) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tp: *const TABLE = 0 as *const TABLE;
    let mut i: libc::c_int = 0;
    let mut abbrev: libc::c_int = 0;
    /* YYDEFGOTO[NTERM-NUM].  */
    /* Make it lowercase. */
    p = buff;
    while *p != 0 {
        if *p as libc::c_uchar as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            *p = tolower(*p as libc::c_uchar as libc::c_int) as libc::c_char
        }
        p = p.offset(1)
    }
    if strcmp(buff, b"am\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(buff, b"a.m.\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        gd_lval.Meridian = MERam;
        return tMERIDIAN as libc::c_int;
    }
    if strcmp(buff, b"pm\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(buff, b"p.m.\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        gd_lval.Meridian = MERpm;
        return tMERIDIAN as libc::c_int;
        /* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
        symbol of state STATE-NUM.  */
    }
    /* See if we have an abbreviation for a month. */
    if strlen(buff) == 3 as libc::c_int as libc::c_ulong {
        abbrev = 1 as libc::c_int
    } else if strlen(buff) == 4 as libc::c_int as libc::c_ulong
        && *buff.offset(3 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        abbrev = 1 as libc::c_int;
        *buff.offset(3 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char
    } else {
        abbrev = 0 as libc::c_int
    }
    /* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
    tp = MonthDayTable.as_ptr();
    while !(*tp).name.is_null() {
        if abbrev != 0 {
            if strncmp(buff, (*tp).name, 3 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
                gd_lval.Number = (*tp).value;
                return (*tp).type_0;
            }
        } else if strcmp(buff, (*tp).name) == 0 as libc::c_int {
            /* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
            gd_lval.Number = (*tp).value;
            return (*tp).type_0;
        }
        tp = tp.offset(1)
    }
    tp = TimezoneTable.as_ptr();
    while !(*tp).name.is_null() {
        if strcmp(buff, (*tp).name) == 0 as libc::c_int {
            gd_lval.Number = (*tp).value;
            return (*tp).type_0;
        }
        tp = tp.offset(1)
    }
    if strcmp(buff, b"dst\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return tDST as libc::c_int;
    }
    tp = UnitsTable.as_ptr();
    while !(*tp).name.is_null() {
        if strcmp(buff, (*tp).name) == 0 as libc::c_int {
            gd_lval.Number = (*tp).value;
            return (*tp).type_0;
        }
        tp = tp.offset(1)
    }
    /* Strip off any plural and try the units table again. */
    i = strlen(buff).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if *buff.offset(i as isize) as libc::c_int == 's' as i32 {
        *buff.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
        tp = UnitsTable.as_ptr();
        while !(*tp).name.is_null() {
            if strcmp(buff, (*tp).name) == 0 as libc::c_int {
                gd_lval.Number = (*tp).value;
                return (*tp).type_0;
            }
            tp = tp.offset(1)
        }
        *buff.offset(i as isize) = 's' as i32 as libc::c_char
        /* Put back for "this" in OtherTable. */
    }
    tp = OtherTable.as_ptr();
    while !(*tp).name.is_null() {
        if strcmp(buff, (*tp).name) == 0 as libc::c_int {
            gd_lval.Number = (*tp).value;
            /* Error token number */
            return (*tp).type_0;
        }
        tp = tp.offset(1)
    }
    /* Military timezones. */
    if *buff.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        && (*buff as libc::c_uchar as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
            && *(*__ctype_b_loc()).offset(*buff as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0)
    {
        /* Enable debugging if requested.  */
        tp = MilitaryTable.as_ptr();
        while !(*tp).name.is_null() {
            if strcmp(buff, (*tp).name) == 0 as libc::c_int {
                gd_lval.Number = (*tp).value;
                return (*tp).type_0;
            }
            tp = tp.offset(1)
        }
    }
    /* Drop out any periods and try the timezone table again. */
    i = 0 as libc::c_int;
    q = buff;
    p = q;
    while *q != 0 {
        if *q as libc::c_int != '.' as i32 {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = *q
        } else {
            i += 1
        }
        q = q.offset(1)
    }
    *p = '\u{0}' as i32 as libc::c_char;
    if i != 0 {
        tp = TimezoneTable.as_ptr();
        while !(*tp).name.is_null() {
            if strcmp(buff, (*tp).name) == 0 as libc::c_int {
                gd_lval.Number = (*tp).value;
                return (*tp).type_0;
            }
            tp = tp.offset(1)
        }
    }
    return tID as libc::c_int;
}
static mut gd_yytable: [yytype_uint8; 51] = [
    2 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
];
static mut gd_yycheck: [yytype_uint8; 51] = [
    0 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
];
static mut yystos: [yytype_uint8; 61] = [
    0 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
];
static mut gd_r1: [yytype_uint8; 52] = [
    0 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
];
static mut gd_r2: [yytype_uint8; 52] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
];
unsafe extern "C" fn gd_lex() -> libc::c_int {
    let mut c: libc::c_uchar = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buff: [libc::c_char; 20] = [0; 20];
    let mut Count: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    loop
    /* skip the '-' sign */
    {
        while *yyInput as libc::c_uchar as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
            && *(*__ctype_b_loc()).offset(*yyInput as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            yyInput = yyInput.offset(1)
        }
        c = *yyInput as libc::c_uchar;
        if (c as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint
            || c as libc::c_int == '-' as i32
            || c as libc::c_int == '+' as i32
        {
            if c as libc::c_int == '-' as i32 || c as libc::c_int == '+' as i32 {
                sign = if c as libc::c_int == '-' as i32 {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
                yyInput = yyInput.offset(1);
                if !((*yyInput as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint)
                {
                    continue;
                }
            } else {
                sign = 0 as libc::c_int
            }
            gd_lval.Number = 0 as libc::c_int;
            loop {
                let fresh1 = yyInput;
                yyInput = yyInput.offset(1);
                c = *fresh1 as libc::c_uchar;
                if !((c as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint)
                {
                    break;
                }
                gd_lval.Number = 10 as libc::c_int * gd_lval.Number + c as libc::c_int - '0' as i32
            }
            yyInput = yyInput.offset(-1);
            if sign < 0 as libc::c_int {
                gd_lval.Number = -gd_lval.Number
            }
            return if sign != 0 {
                tSNUMBER as libc::c_int
            } else {
                tUNUMBER as libc::c_int
            };
        } else {
            if c as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
                && *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
            {
                p = buff.as_mut_ptr();
                loop {
                    let fresh2 = yyInput;
                    yyInput = yyInput.offset(1);
                    c = *fresh2 as libc::c_uchar;
                    if !(c as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
                        && *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        || c as libc::c_int == '.' as i32)
                    {
                        break;
                    }
                    if p < &mut *buff.as_mut_ptr().offset(
                        (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_char
                    {
                        let fresh3 = p;
                        p = p.offset(1);
                        *fresh3 = c as libc::c_char
                    }
                }
                *p = '\u{0}' as i32 as libc::c_char;
                yyInput = yyInput.offset(-1);
                return LookupWord(buff.as_mut_ptr());
            }
            if c as libc::c_int != '(' as i32 {
                let fresh4 = yyInput;
                yyInput = yyInput.offset(1);
                return *fresh4 as libc::c_int;
            }
            Count = 0 as libc::c_int;
            loop {
                let fresh5 = yyInput;
                yyInput = yyInput.offset(1);
                c = *fresh5 as libc::c_uchar;
                if c as libc::c_int == '\u{0}' as i32 {
                    return c as libc::c_int;
                }
                if c as libc::c_int == '(' as i32 {
                    Count += 1
                } else if c as libc::c_int == ')' as i32 {
                    Count -= 1
                }
                if !(Count > 0 as libc::c_int) {
                    break;
                }
            }
        }
    }
}
/* Yield A - B, measured in seconds.  */
unsafe extern "C" fn difftm(mut a: *mut tm, mut b: *mut tm) -> libc::c_long {
    let mut ay: libc::c_int = (*a).tm_year + (1900 as libc::c_int - 1 as libc::c_int);
    let mut by: libc::c_int = (*b).tm_year + (1900 as libc::c_int - 1 as libc::c_int);
    let mut days: libc::c_long =
        ((*a).tm_yday - (*b).tm_yday + ((ay >> 2 as libc::c_int) - (by >> 2 as libc::c_int))
            - (ay / 100 as libc::c_int - by / 100 as libc::c_int)
            + ((ay / 100 as libc::c_int >> 2 as libc::c_int)
                - (by / 100 as libc::c_int >> 2 as libc::c_int))) as libc::c_long
            + (ay - by) as libc::c_long * 365 as libc::c_int as libc::c_long;
    return 60 as libc::c_int as libc::c_long
        * (60 as libc::c_int as libc::c_long
            * (24 as libc::c_int as libc::c_long * days
                + ((*a).tm_hour - (*b).tm_hour) as libc::c_long)
            + ((*a).tm_min - (*b).tm_min) as libc::c_long)
        + ((*a).tm_sec - (*b).tm_sec) as libc::c_long;
}
/*  Copyright (C) 1995 Free Software Foundation, Inc.

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2, or (at your option)
any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software Foundation,
Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA. */
/* Modified from the original to add stdlib.h and string.h */
#[no_mangle]
pub unsafe extern "C" fn get_date(mut p: *const libc::c_char, mut now: *const time_t) -> time_t {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut tm0: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut tmp: *mut tm = 0 as *mut tm;
    let mut Start: time_t = 0;
    yyInput = p;
    Start = if !now.is_null() {
        *now
    } else {
        time(0 as *mut libc::c_void as *mut time_t)
    };
    tmp = localtime(&mut Start);
    if tmp.is_null() {
        return -(1 as libc::c_int) as time_t;
    }
    yyYear = (*tmp).tm_year + 1900 as libc::c_int;
    yyMonth = (*tmp).tm_mon + 1 as libc::c_int;
    yyDay = (*tmp).tm_mday;
    yyHour = (*tmp).tm_hour;
    yyMinutes = (*tmp).tm_min;
    yySeconds = (*tmp).tm_sec;
    tm.tm_isdst = (*tmp).tm_isdst;
    yyMeridian = MER24;
    yyRelSeconds = 0 as libc::c_int;
    yyRelMinutes = 0 as libc::c_int;
    yyRelHour = 0 as libc::c_int;
    yyRelDay = 0 as libc::c_int;
    /* YYINITDEPTH -- initial size of the parser's stacks.  */
    yyRelMonth = 0 as libc::c_int;
    yyRelYear = 0 as libc::c_int;
    yyHaveDate = 0 as libc::c_int;
    yyHaveDay = 0 as libc::c_int;
    yyHaveRel = 0 as libc::c_int;
    /* YYMAXDEPTH -- maximum size the stacks can grow to (effective only
    if the built-in stack extension method is used).

    Do not make this value too large; the results are undefined if
    YYSTACK_ALLOC_MAXIMUM < YYSTACK_BYTES (YYMAXDEPTH)
    evaluated with infinite-precision integer arithmetic.  */
    yyHaveTime = 0 as libc::c_int;
    yyHaveZone = 0 as libc::c_int;
    if gd_parse() != 0
        || yyHaveTime > 1 as libc::c_int
        || yyHaveZone > 1 as libc::c_int
        || yyHaveDate > 1 as libc::c_int
        || yyHaveDay > 1 as libc::c_int
    {
        return -(1 as libc::c_int) as time_t;
    }
    tm.tm_year = ToYear(yyYear) - 1900 as libc::c_int + yyRelYear;
    tm.tm_mon = yyMonth - 1 as libc::c_int + yyRelMonth;
    tm.tm_mday = yyDay + yyRelDay;
    if yyHaveTime != 0 || yyHaveRel != 0 && yyHaveDate == 0 && yyHaveDay == 0 {
        tm.tm_hour = ToHour(yyHour, yyMeridian as libc::c_uint);
        if tm.tm_hour < 0 as libc::c_int {
            return -(1 as libc::c_int) as time_t;
        }
        tm.tm_min = yyMinutes;
        tm.tm_sec = yySeconds
    } else {
        tm.tm_sec = 0 as libc::c_int;
        tm.tm_min = tm.tm_sec;
        tm.tm_hour = tm.tm_min
    }
    tm.tm_hour += yyRelHour;
    tm.tm_min += yyRelMinutes;
    tm.tm_sec += yyRelSeconds;
    /* Let mktime deduce tm_isdst if we have an absolute timestamp,
    or if the relative timestamp mentions days, months, or years.  */
    if yyHaveDate | yyHaveDay | yyHaveTime | yyRelDay | yyRelMonth | yyRelYear != 0 {
        tm.tm_isdst = -(1 as libc::c_int)
    }
    tm0 = tm;
    Start = mktime(&mut tm);
    if Start == -(1 as libc::c_int) as time_t {
        /* Guard against falsely reporting errors near the time_t boundaries
        when parsing times in other time zones.  For example, if the min
        time_t value is 1970-01-01 00:00:00 UTC and we are 8 hours ahead
        of UTC, then the min localtime value is 1970-01-01 08:00:00; if
        we apply mktime to 1970-01-01 00:00:00 we will get an error, so
        we apply mktime to 1970-01-02 08:00:00 instead and adjust the time
        zone by 24 hours to compensate.  This algorithm assumes that
        there is no DST transition within a day of the time_t boundaries.  */
        if yyHaveZone != 0 {
            tm = tm0; /* time_t overflow */
            if tm.tm_year <= 1970 as libc::c_int - 1900 as libc::c_int {
                tm.tm_mday += 1;
                yyTimezone -= 24 as libc::c_int * 60 as libc::c_int
            } else {
                tm.tm_mday -= 1;
                yyTimezone += 24 as libc::c_int * 60 as libc::c_int
            }
            Start = mktime(&mut tm)
        }
        if Start == -(1 as libc::c_int) as time_t {
            return Start;
        }
    }
    if yyHaveDay != 0 && yyHaveDate == 0 {
        tm.tm_mday += (yyDayNumber - tm.tm_wday + 7 as libc::c_int) % 7 as libc::c_int
            + 7 as libc::c_int
                * (yyDayOrdinal - ((0 as libc::c_int) < yyDayOrdinal) as libc::c_int);
        Start = mktime(&mut tm);
        if Start == -(1 as libc::c_int) as time_t {
            return Start;
        }
    }
    if yyHaveZone != 0 {
        let mut delta: libc::c_long = 0;
        let mut gmt: *mut tm = gmtime(&mut Start);
        if gmt.is_null() {
            return -(1 as libc::c_int) as time_t;
        }
        delta = yyTimezone as libc::c_long * 60 as libc::c_long + difftm(&mut tm, gmt);
        if (Start + delta < Start) as libc::c_int
            != (delta < 0 as libc::c_int as libc::c_long) as libc::c_int
        {
            return -(1 as libc::c_int) as time_t;
        }
        Start += delta
    }
    return Start;
}
/* defined (TEST) */
/* YYERROR_VERBOSE */
/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yytype: libc::c_int,
    mut yyvaluep: *mut YYSTYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\x00" as *const u8 as *const libc::c_char
    };
}
/* The lookahead symbol.  */
#[no_mangle]
pub static mut gd_char: libc::c_int = 0;
/* The semantic value of the lookahead symbol.  */
#[no_mangle]
pub static mut gd_lval: YYSTYPE = YYSTYPE { Number: 0 };
/* Number of syntax errors so far.  */
#[no_mangle]
pub static mut gd_nerrs: libc::c_int = 0;
/*----------.
| yyparse.  |
`----------*/
#[no_mangle]
pub unsafe extern "C" fn gd_parse() -> libc::c_int {
    let mut current_block: u64;
    let mut gd_state: libc::c_int = 0;
    /* Number of tokens to shift before error messages enabled.  */
    let mut yyerrstatus: libc::c_int = 0;
    /* The stacks and their tools:
    'yyss': related to states.
    'yyvs': related to semantic values.

    Refer to the stacks through separate pointers, to allow yyoverflow
    to reallocate them elsewhere.  */
    /* The state stack.  */
    let mut yyssa: [yytype_int16; 200] = [0; 200];
    let mut yyss: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyssp: *mut yytype_int16 = 0 as *mut yytype_int16;
    /* The semantic value stack.  */
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE { Number: 0 }; 200];
    let mut yyvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystacksize: libc::c_ulong = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    /* Lookahead token as an internal (translated) token number.  */
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    /* The variables used to return semantic value and location from the
    action routines.  */
    let mut gd_val: YYSTYPE = YYSTYPE { Number: 0 };
    /* The number of symbols on the RHS of the reduced rule.
    Keep to zero when no symbol should be popped.  */
    let mut gd_yylen: libc::c_int = 0 as libc::c_int; /* Cause a token to be read.  */
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int as libc::c_ulong;
    gd_state = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    gd_nerrs = 0 as libc::c_int;
    gd_char = -(2 as libc::c_int);
    's_88: loop
    /*--------------------------------------------------------------------.
    | yynewstate -- set current state (the top of the stack) to yystate.  |
    `--------------------------------------------------------------------*/
    {
        (0 as libc::c_int != 0 && (0 as libc::c_int <= gd_state && gd_state < 61 as libc::c_int))
            as libc::c_int;
        *yyssp = gd_state as yytype_int16;
        if yyss
            .offset(yystacksize as isize)
            .offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            /* Get the current used size of the three stacks, in elements.  */
            let mut yysize: libc::c_ulong = (yyssp.wrapping_offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong;
            /* defined YYSTACK_RELOCATE */
            /* Extend the stack our own way.  */
            if 10000 as libc::c_int as libc::c_ulong <= yystacksize {
                current_block = 7469032322570361295;
                break;
            }
            yystacksize = yystacksize.wrapping_mul(2 as libc::c_int as libc::c_ulong);
            if (10000 as libc::c_int as libc::c_ulong) < yystacksize {
                yystacksize = 10000 as libc::c_int as libc::c_ulong
            }
            let mut yyss1: *mut yytype_int16 = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                yystacksize
                    .wrapping_mul(
                        (::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                            .wrapping_add(::std::mem::size_of::<YYSTYPE>() as libc::c_ulong),
                    )
                    .wrapping_add(
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 7469032322570361295;
                break;
            }
            let mut yynewbytes: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yytype_int16 as *mut libc::c_void,
                yyss as *const libc::c_void,
                yysize.wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                .wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr.offset(
                yynewbytes.wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong) as isize,
            );
            let mut yynewbytes_0: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut YYSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                yysize.wrapping_mul(::std::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                .wrapping_mul(::std::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr.offset(
                yynewbytes_0.wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                    as isize,
            );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss
                .offset(yysize as isize)
                .offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs
                .offset(yysize as isize)
                .offset(-(1 as libc::c_int as isize));
            if yyss
                .offset(yystacksize as isize)
                .offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 926614003985312789;
                break;
            }
        }
        /* !defined yyoverflow && !defined YYSTACK_RELOCATE */
        if gd_state == 2 as libc::c_int {
            /*-------------------------------------.
            | yyacceptlab -- YYACCEPT comes here.  |
            `-------------------------------------*/
            yyresult = 0 as libc::c_int;
            current_block = 13408375855341293595;
            break;
        } else {
            /*-----------.
            | yybackup.  |
            `-----------*/
            /* Do appropriate processing given the current state.  Read a
            lookahead token if we need one and don't already have one.  */
            /* First try to decide what to do without reference to lookahead token.  */
            yyn = gd_pact[gd_state as usize] as libc::c_int;
            if yyn == -(20 as libc::c_int) {
                current_block = 13115551051516724588;
            } else {
                /* Not known => get a lookahead token if don't already have one.  */
                /* YYCHAR is either YYEMPTY or YYEOF or a valid lookahead symbol.  */
                if gd_char == -(2 as libc::c_int) {
                    gd_char = gd_lex()
                }
                if gd_char <= 0 as libc::c_int {
                    yytoken = 0 as libc::c_int;
                    gd_char = yytoken
                } else {
                    yytoken = if gd_char as libc::c_uint <= 273 as libc::c_int as libc::c_uint {
                        yytranslate[gd_char as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    }
                }
                /* If the proper action on seeing token YYTOKEN is to reduce or to
                detect an error, take that action.  */
                yyn += yytoken;
                if yyn < 0 as libc::c_int
                    || (50 as libc::c_int) < yyn
                    || gd_yycheck[yyn as usize] as libc::c_int != yytoken
                {
                    current_block = 13115551051516724588;
                } else {
                    yyn = gd_yytable[yyn as usize] as libc::c_int;
                    if yyn <= 0 as libc::c_int {
                        yyn = -yyn;
                        current_block = 9525926865631088734;
                    } else {
                        /* Count tokens shifted since error; after three, turn off error
                        status.  */
                        if yyerrstatus != 0 {
                            yyerrstatus -= 1
                        }
                        /* Shift the lookahead token.  */
                        /* Discard the shifted token.  */
                        gd_char = -(2 as libc::c_int);
                        gd_state = yyn;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = gd_lval;
                        current_block = 373966280438382103;
                    }
                }
            }
            match current_block {
                13115551051516724588 =>
                /*-----------------------------------------------------------.
                | yydefault -- do the default action for the current state.  |
                `-----------------------------------------------------------*/
                {
                    yyn = yydefact[gd_state as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        /*--------------------------------------.
                        | yyerrlab -- here on detecting error.  |
                        `--------------------------------------*/
                        /* Make sure we have latest lookahead translation.  See comments at
                        user semantic actions for why this is necessary.  */
                        yytoken = if gd_char == -(2 as libc::c_int) {
                            -(2 as libc::c_int)
                        } else if gd_char as libc::c_uint <= 273 as libc::c_int as libc::c_uint {
                            yytranslate[gd_char as usize] as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                        /* If not already recovering from an error, report this error.  */
                        if yyerrstatus == 0 {
                            gd_nerrs += 1;
                            gd_error(
                                b"syntax error\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            /* If just tried and failed to reuse lookahead token after an
                            error, discard it.  */
                            if gd_char <= 0 as libc::c_int {
                                /* Return failure if at end of input.  */
                                if gd_char == 0 as libc::c_int {
                                    current_block = 926614003985312789;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\x00" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut gd_lval,
                                );
                                gd_char = -(2 as libc::c_int)
                            }
                        }
                        /* Else will try to reuse lookahead token after shifting the error
                        token.  */
                        /*-------------------------------------------------------------.
                        | yyerrlab1 -- common code for both syntax error and YYERROR.  |
                        `-------------------------------------------------------------*/
                        yyerrstatus = 3 as libc::c_int; /* Each real token shifted decrements this.  */
                        loop {
                            yyn = gd_pact[gd_state as usize] as libc::c_int;
                            if !(yyn == -(20 as libc::c_int)) {
                                yyn += 1 as libc::c_int;
                                if 0 as libc::c_int <= yyn
                                    && yyn <= 50 as libc::c_int
                                    && gd_yycheck[yyn as usize] as libc::c_int == 1 as libc::c_int
                                {
                                    yyn = gd_yytable[yyn as usize] as libc::c_int;
                                    if (0 as libc::c_int) < yyn {
                                        break;
                                    }
                                }
                            }
                            /* Pop the current state because it cannot handle the error token.  */
                            if yyssp == yyss {
                                current_block = 926614003985312789;
                                break 's_88;
                            }
                            yydestruct(
                                b"Error: popping\x00" as *const u8 as *const libc::c_char,
                                yystos[gd_state as usize] as libc::c_int,
                                yyvsp,
                            );
                            yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                            yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                            gd_state = *yyssp as libc::c_int
                        }
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = gd_lval;
                        /* Shift the error token.  */
                        gd_state = yyn;
                        current_block = 373966280438382103;
                    } else {
                        current_block = 9525926865631088734;
                    }
                }
                _ => {}
            }
            match current_block {
                9525926865631088734 =>
                /*-----------------------------.
                | yyreduce -- do a reduction.  |
                `-----------------------------*/
                                    /* yyn is the number of a rule to reduce with.  */
                {
                    gd_yylen = gd_r2[yyn as usize] as libc::c_int;
                    /* If YYLEN is nonzero, implement the default value of the action:
                    '$$ = $1'.

                    Otherwise, the following line sets YYVAL to garbage.
                    This behavior is undocumented and Bison
                    users should not rely upon it.  Assigning to YYVAL
                    unconditionally makes the parser a bit smaller, and it avoids a
                    GCC warning that YYVAL may be used uninitialized.  */
                    gd_val = *yyvsp.offset((1 as libc::c_int - gd_yylen) as isize);
                    match yyn {
                        4 => yyHaveTime += 1,
                        5 => yyHaveZone += 1,
                        6 => yyHaveDate += 1,
                        7 => yyHaveDay += 1,
                        8 => yyHaveRel += 1,
                        10 => {
                            yyHour = (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number;
                            yyMinutes = 0 as libc::c_int;
                            yySeconds = 0 as libc::c_int;
                            yyMeridian = (*yyvsp.offset(0 as libc::c_int as isize)).Meridian
                        }
                        11 => {
                            yyHour = (*yyvsp.offset(-(3 as libc::c_int) as isize)).Number;
                            yyMinutes = (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number;
                            yySeconds = 0 as libc::c_int;
                            yyMeridian = (*yyvsp.offset(0 as libc::c_int as isize)).Meridian
                        }
                        12 => {
                            yyHour = (*yyvsp.offset(-(3 as libc::c_int) as isize)).Number;
                            yyMinutes = (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number;
                            yyMeridian = MER24;
                            yyHaveZone += 1;
                            yyTimezone = if (*yyvsp.offset(0 as libc::c_int as isize)).Number
                                < 0 as libc::c_int
                            {
                                (-(*yyvsp.offset(0 as libc::c_int as isize)).Number
                                    % 100 as libc::c_int)
                                    + -(*yyvsp.offset(0 as libc::c_int as isize)).Number
                                        / 100 as libc::c_int
                                        * 60 as libc::c_int
                            } else {
                                -((*yyvsp.offset(0 as libc::c_int as isize)).Number
                                    % 100 as libc::c_int
                                    + (*yyvsp.offset(0 as libc::c_int as isize)).Number
                                        / 100 as libc::c_int
                                        * 60 as libc::c_int)
                            }
                        }
                        13 => {
                            yyHour = (*yyvsp.offset(-(5 as libc::c_int) as isize)).Number;
                            yyMinutes = (*yyvsp.offset(-(3 as libc::c_int) as isize)).Number;
                            yySeconds = (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number;
                            yyMeridian = (*yyvsp.offset(0 as libc::c_int as isize)).Meridian
                        }
                        14 => {
                            yyHour = (*yyvsp.offset(-(5 as libc::c_int) as isize)).Number;
                            yyMinutes = (*yyvsp.offset(-(3 as libc::c_int) as isize)).Number;
                            yySeconds = (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number;
                            yyMeridian = MER24;
                            yyHaveZone += 1;
                            yyTimezone = if (*yyvsp.offset(0 as libc::c_int as isize)).Number
                                < 0 as libc::c_int
                            {
                                (-(*yyvsp.offset(0 as libc::c_int as isize)).Number
                                    % 100 as libc::c_int)
                                    + -(*yyvsp.offset(0 as libc::c_int as isize)).Number
                                        / 100 as libc::c_int
                                        * 60 as libc::c_int
                            } else {
                                -((*yyvsp.offset(0 as libc::c_int as isize)).Number
                                    % 100 as libc::c_int
                                    + (*yyvsp.offset(0 as libc::c_int as isize)).Number
                                        / 100 as libc::c_int
                                        * 60 as libc::c_int)
                            }
                        }
                        15 => yyTimezone = (*yyvsp.offset(0 as libc::c_int as isize)).Number,
                        16 => {
                            yyTimezone = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                                - 60 as libc::c_int
                        }
                        17 => {
                            yyTimezone = (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                - 60 as libc::c_int
                        }
                        18 => {
                            yyDayOrdinal = 1 as libc::c_int;
                            yyDayNumber = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        19 => {
                            yyDayOrdinal = 1 as libc::c_int;
                            yyDayNumber = (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                        }
                        20 => {
                            yyDayOrdinal = (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number;
                            yyDayNumber = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        21 => {
                            yyMonth = (*yyvsp.offset(-(2 as libc::c_int) as isize)).Number;
                            yyDay = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        22 => {
                            if (*yyvsp.offset(-(4 as libc::c_int) as isize)).Number
                                >= 1000 as libc::c_int
                            {
                                yyYear = (*yyvsp.offset(-(4 as libc::c_int) as isize)).Number;
                                yyMonth = (*yyvsp.offset(-(2 as libc::c_int) as isize)).Number;
                                yyDay = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                            } else {
                                yyMonth = (*yyvsp.offset(-(4 as libc::c_int) as isize)).Number;
                                yyDay = (*yyvsp.offset(-(2 as libc::c_int) as isize)).Number;
                                yyYear = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                            }
                        }
                        23 => {
                            yyYear = (*yyvsp.offset(-(2 as libc::c_int) as isize)).Number;
                            yyMonth = -(*yyvsp.offset(-(1 as libc::c_int) as isize)).Number;
                            yyDay = -(*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        24 => {
                            yyDay = (*yyvsp.offset(-(2 as libc::c_int) as isize)).Number;
                            yyMonth = (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number;
                            yyYear = -(*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        25 => {
                            yyMonth = (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number;
                            yyDay = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        26 => {
                            yyMonth = (*yyvsp.offset(-(3 as libc::c_int) as isize)).Number;
                            yyDay = (*yyvsp.offset(-(2 as libc::c_int) as isize)).Number;
                            yyYear = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        27 => {
                            yyMonth = (*yyvsp.offset(0 as libc::c_int as isize)).Number;
                            yyDay = (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                        }
                        28 => {
                            yyMonth = (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number;
                            yyDay = (*yyvsp.offset(-(2 as libc::c_int) as isize)).Number;
                            yyYear = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        29 => {
                            yyRelSeconds = -yyRelSeconds;
                            yyRelMinutes = -yyRelMinutes;
                            yyRelHour = -yyRelHour;
                            yyRelDay = -yyRelDay;
                            yyRelMonth = -yyRelMonth;
                            yyRelYear = -yyRelYear
                        }
                        31 => {
                            yyRelYear += (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                * (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        32 => {
                            yyRelYear += (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                * (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        33 => yyRelYear += (*yyvsp.offset(0 as libc::c_int as isize)).Number,
                        34 => {
                            yyRelMonth += (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                * (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        35 => {
                            yyRelMonth += (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                * (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        36 => yyRelMonth += (*yyvsp.offset(0 as libc::c_int as isize)).Number,
                        37 => {
                            yyRelDay += (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                * (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        38 => {
                            yyRelDay += (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                * (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        39 => yyRelDay += (*yyvsp.offset(0 as libc::c_int as isize)).Number,
                        40 => {
                            yyRelHour += (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                * (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        41 => {
                            yyRelHour += (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                * (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        42 => yyRelHour += (*yyvsp.offset(0 as libc::c_int as isize)).Number,
                        43 => {
                            yyRelMinutes += (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                * (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        44 => {
                            yyRelMinutes += (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                * (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        45 => yyRelMinutes += (*yyvsp.offset(0 as libc::c_int as isize)).Number,
                        46 => {
                            yyRelSeconds += (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                * (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        47 => {
                            yyRelSeconds += (*yyvsp.offset(-(1 as libc::c_int) as isize)).Number
                                * (*yyvsp.offset(0 as libc::c_int as isize)).Number
                        }
                        48 => yyRelSeconds += (*yyvsp.offset(0 as libc::c_int as isize)).Number,
                        49 => {
                            if yyHaveTime != 0 && yyHaveDate != 0 && yyHaveRel == 0 {
                                yyYear = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                            } else if (*yyvsp.offset(0 as libc::c_int as isize)).Number
                                > 10000 as libc::c_int
                            {
                                yyHaveDate += 1;
                                yyDay = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                                    % 100 as libc::c_int;
                                yyMonth = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                                    / 100 as libc::c_int
                                    % 100 as libc::c_int;
                                yyYear = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                                    / 10000 as libc::c_int
                            } else {
                                yyHaveTime += 1;
                                if (*yyvsp.offset(0 as libc::c_int as isize)).Number
                                    < 100 as libc::c_int
                                {
                                    yyHour = (*yyvsp.offset(0 as libc::c_int as isize)).Number;
                                    yyMinutes = 0 as libc::c_int
                                } else {
                                    yyHour = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                                        / 100 as libc::c_int;
                                    yyMinutes = (*yyvsp.offset(0 as libc::c_int as isize)).Number
                                        % 100 as libc::c_int
                                }
                                yySeconds = 0 as libc::c_int;
                                yyMeridian = MER24
                            }
                        }
                        50 => gd_val.Meridian = MER24,
                        51 => gd_val.Meridian = (*yyvsp.offset(0 as libc::c_int as isize)).Meridian,
                        _ => {}
                    }
                    /* User semantic actions sometimes alter yychar, and that requires
                    that yytoken be updated with the new translation.  We take the
                    approach of translating immediately before every use of yytoken.
                    One alternative is translating here after every semantic action,
                    but that translation would be missed if the semantic action invokes
                    YYABORT, YYACCEPT, or YYERROR immediately after altering yychar or
                    if it invokes YYBACKUP.  In the case of YYABORT or YYACCEPT, an
                    incorrect destructor might then be invoked immediately.  In the
                    case of YYERROR or YYBACKUP, subsequent parser actions might lead
                    to an incorrect destructor call or verbose syntax error message
                    before the lookahead is translated.  */
                    yyvsp = yyvsp.offset(-(gd_yylen as isize));
                    yyssp = yyssp.offset(-(gd_yylen as isize));
                    gd_yylen = 0 as libc::c_int;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = gd_val;
                    /* Now 'shift' the result of the reduction.  Determine what state
                    that goes to, based on the state we popped back to and the rule
                    number reduced by.  */
                    let gd_yylhs: libc::c_int =
                        gd_r1[yyn as usize] as libc::c_int - 22 as libc::c_int;
                    let yyi: libc::c_int =
                        yypgoto[gd_yylhs as usize] as libc::c_int + *yyssp as libc::c_int;
                    gd_state = if 0 as libc::c_int <= yyi
                        && yyi <= 50 as libc::c_int
                        && gd_yycheck[yyi as usize] as libc::c_int == *yyssp as libc::c_int
                    {
                        gd_yytable[yyi as usize] as libc::c_int
                    } else {
                        yydefgoto[gd_yylhs as usize] as libc::c_int
                    }
                }
                _ => {}
            }
            /*------------------------------------------------------------.
            | yynewstate -- push a new state, which is found in yystate.  |
            `------------------------------------------------------------*/
            /* In all cases, when you get here, the value and location stacks
            have just been pushed.  So pushing a state here evens the stacks.  */
            yyssp = yyssp.offset(1)
        }
    }
    match current_block {
        7469032322570361295 =>
        /*-------------------------------------------------.
        | yyexhaustedlab -- memory exhaustion comes here.  |
        `-------------------------------------------------*/
        {
            gd_error(
                b"memory exhausted\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            yyresult = 2 as libc::c_int
        }
        926614003985312789 =>
        /*-----------------------------------.
        | yyabortlab -- YYABORT comes here.  |
        `-----------------------------------*/
        {
            yyresult = 1 as libc::c_int
        }
        _ => {}
    }
    /* Fall through.  */
    /*-----------------------------------------------------.
    | yyreturn -- parsing is finished, return the result.  |
    `-----------------------------------------------------*/
    if gd_char != -(2 as libc::c_int) {
        /* Make sure we have latest lookahead translation.  See comments at
        user semantic actions for why this is necessary.  */
        yytoken = if gd_char as libc::c_uint <= 273 as libc::c_int as libc::c_uint {
            yytranslate[gd_char as usize] as libc::c_int
        } else {
            2 as libc::c_int
        };
        yydestruct(
            b"Cleanup: discarding lookahead\x00" as *const u8 as *const libc::c_char,
            yytoken,
            &mut gd_lval,
        );
    }
    /* Do not reclaim the symbols of the rule whose action triggered
    this YYABORT or YYACCEPT.  */
    yyvsp = yyvsp.offset(-(gd_yylen as isize));
    yyssp = yyssp.offset(-(gd_yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\x00" as *const u8 as *const libc::c_char,
            yystos[*yyssp as usize] as libc::c_int,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize))
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
