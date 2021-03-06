// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use once_cell::unsync::Lazy;
use pyo3::ffi::*;
use std::os::raw::c_char;
use std::ptr::NonNull;
use std::sync::Once;

pub static mut HASH_SEED: u64 = 0;

pub static mut NONE: *mut PyObject = 0 as *mut PyObject;
pub static mut TRUE: *mut PyObject = 0 as *mut PyObject;
pub static mut FALSE: *mut PyObject = 0 as *mut PyObject;

pub static mut STR_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut INT_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut BOOL_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut NONE_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut FLOAT_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut LIST_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut DICT_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut DATETIME_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut DATE_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut TIME_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut TUPLE_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut UUID_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut ENUM_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut ARRAY_TYPE: Lazy<Option<NonNull<PyTypeObject>>> =
    Lazy::new(|| unsafe { look_up_array_type() });
pub static mut FIELD_TYPE: Lazy<NonNull<PyObject>> = Lazy::new(|| unsafe { look_up_field_type() });

pub static mut BYTES_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut BYTEARRAY_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;

pub static mut INT_ATTR_STR: *mut PyObject = 0 as *mut PyObject;
pub static mut UTCOFFSET_METHOD_STR: *mut PyObject = 0 as *mut PyObject;
pub static mut NORMALIZE_METHOD_STR: *mut PyObject = 0 as *mut PyObject;
pub static mut CONVERT_METHOD_STR: *mut PyObject = 0 as *mut PyObject;
pub static mut DST_STR: *mut PyObject = 0 as *mut PyObject;
pub static mut DICT_STR: *mut PyObject = 0 as *mut PyObject;
pub static mut DATACLASS_FIELDS_STR: *mut PyObject = 0 as *mut PyObject;
pub static mut FIELD_TYPE_STR: *mut PyObject = 0 as *mut PyObject;
pub static mut ARRAY_STRUCT_STR: *mut PyObject = 0 as *mut PyObject;
pub static mut VALUE_STR: *mut PyObject = 0 as *mut PyObject;
pub static mut STR_HASH_FUNCTION: Option<hashfunc> = None;
pub static mut DEFAULT: *mut PyObject = 0 as *mut PyObject;
pub static mut OPTION: *mut PyObject = 0 as *mut PyObject;

#[allow(non_upper_case_globals)]
pub static mut JsonEncodeError: *mut PyObject = 0 as *mut PyObject;
#[allow(non_upper_case_globals)]
pub static mut JsonDecodeError: *mut PyObject = 0 as *mut PyObject;

static INIT: Once = Once::new();

pub fn init_typerefs() {
    INIT.call_once(|| unsafe {
        assert!(crate::decode::KEY_MAP
            .set(crate::decode::KeyMap::default())
            .is_ok());
        PyDateTime_IMPORT();
        NONE = Py_None();
        TRUE = Py_True();
        FALSE = Py_False();
        let unicode = PyUnicode_New(0, 255);
        STR_TYPE = (*unicode).ob_type;
        STR_HASH_FUNCTION = (*((*unicode).ob_type)).tp_hash;
        BYTES_TYPE = (*PyBytes_FromStringAndSize("".as_ptr() as *const c_char, 0)).ob_type;
        BYTEARRAY_TYPE = (*PyByteArray_FromStringAndSize("".as_ptr() as *const c_char, 0)).ob_type;
        DICT_TYPE = (*PyDict_New()).ob_type;
        LIST_TYPE = (*PyList_New(0 as Py_ssize_t)).ob_type;
        TUPLE_TYPE = (*PyTuple_New(0 as Py_ssize_t)).ob_type;
        NONE_TYPE = (*NONE).ob_type;
        BOOL_TYPE = (*TRUE).ob_type;
        INT_TYPE = (*PyLong_FromLongLong(0)).ob_type;
        FLOAT_TYPE = (*PyFloat_FromDouble(0.0)).ob_type;
        DATETIME_TYPE = look_up_datetime_type();
        DATE_TYPE = look_up_date_type();
        TIME_TYPE = look_up_time_type();
        UUID_TYPE = look_up_uuid_type();
        ENUM_TYPE = look_up_enum_type();
        INT_ATTR_STR = PyUnicode_InternFromString("int\0".as_ptr() as *const c_char);
        UTCOFFSET_METHOD_STR = PyUnicode_InternFromString("utcoffset\0".as_ptr() as *const c_char);
        NORMALIZE_METHOD_STR = PyUnicode_InternFromString("normalize\0".as_ptr() as *const c_char);
        CONVERT_METHOD_STR = PyUnicode_InternFromString("convert\0".as_ptr() as *const c_char);
        DST_STR = PyUnicode_InternFromString("dst\0".as_ptr() as *const c_char);
        DICT_STR = PyUnicode_InternFromString("__dict__\0".as_ptr() as *const c_char);
        DATACLASS_FIELDS_STR =
            PyUnicode_InternFromString("__dataclass_fields__\0".as_ptr() as *const c_char);
        FIELD_TYPE_STR = PyUnicode_InternFromString("_field_type\0".as_ptr() as *const c_char);
        ARRAY_STRUCT_STR =
            pyo3::ffi::PyUnicode_InternFromString("__array_struct__\0".as_ptr() as *const c_char);
        VALUE_STR = pyo3::ffi::PyUnicode_InternFromString("value\0".as_ptr() as *const c_char);
        HASH_SEED = (VALUE_STR as u64).wrapping_mul(DICT_TYPE as u64);
        DEFAULT = PyUnicode_InternFromString("default\0".as_ptr() as *const c_char);
        OPTION = PyUnicode_InternFromString("option\0".as_ptr() as *const c_char);
        JsonEncodeError = pyo3::ffi::PyExc_TypeError;
        JsonDecodeError = look_up_json_exc();
    });
}

unsafe fn look_up_json_exc() -> *mut PyObject {
    let module = PyImport_ImportModule("json\0".as_ptr() as *const c_char);
    let module_dict = PyModule_GetDict(module);
    let ptr = PyMapping_GetItemString(module_dict, "JSONDecodeError\0".as_ptr() as *const c_char)
        as *mut PyObject;
    let res = pyo3::ffi::PyErr_NewException(
        "orjson.JSONDecodeError\0".as_ptr() as *const c_char,
        ptr,
        std::ptr::null_mut(),
    );
    Py_DECREF(ptr);
    Py_DECREF(module_dict);
    Py_DECREF(module);
    res
}

unsafe fn look_up_array_type() -> Option<NonNull<PyTypeObject>> {
    let numpy = PyImport_ImportModule("numpy\0".as_ptr() as *const c_char);
    if numpy.is_null() {
        PyErr_Clear();
        return None;
    } else {
        let mod_dict = PyModule_GetDict(numpy);
        let ptr = PyMapping_GetItemString(mod_dict, "ndarray\0".as_ptr() as *const c_char);
        Py_XDECREF(ptr);
        // Py_XDECREF(mod_dict) causes segfault when pytest exits
        Py_XDECREF(numpy);
        Some(NonNull::new_unchecked(ptr as *mut PyTypeObject))
    }
}

unsafe fn look_up_field_type() -> NonNull<PyObject> {
    let module = PyImport_ImportModule("dataclasses\0".as_ptr() as *const c_char);
    let module_dict = PyModule_GetDict(module);
    let ptr = PyMapping_GetItemString(module_dict, "_FIELD\0".as_ptr() as *const c_char)
        as *mut PyTypeObject;
    Py_DECREF(module_dict);
    Py_DECREF(module);
    NonNull::new_unchecked(ptr as *mut PyObject)
}

unsafe fn look_up_enum_type() -> *mut PyTypeObject {
    let module = PyImport_ImportModule("enum\0".as_ptr() as *const c_char);
    let module_dict = PyModule_GetDict(module);
    let ptr = PyMapping_GetItemString(module_dict, "EnumMeta\0".as_ptr() as *const c_char)
        as *mut PyTypeObject;
    Py_DECREF(module_dict);
    Py_DECREF(module);
    ptr
}

unsafe fn look_up_uuid_type() -> *mut PyTypeObject {
    let uuid_mod = PyImport_ImportModule("uuid\0".as_ptr() as *const c_char);
    let uuid_mod_dict = PyModule_GetDict(uuid_mod);
    let uuid = PyMapping_GetItemString(uuid_mod_dict, "NAMESPACE_DNS\0".as_ptr() as *const c_char);
    let ptr = (*uuid).ob_type;
    Py_DECREF(uuid);
    Py_DECREF(uuid_mod_dict);
    Py_DECREF(uuid_mod);
    ptr
}

unsafe fn look_up_datetime_type() -> *mut PyTypeObject {
    let datetime = (PyDateTimeAPI.DateTime_FromDateAndTime)(
        1970,
        1,
        1,
        0,
        0,
        0,
        0,
        NONE,
        PyDateTimeAPI.DateTimeType,
    );
    let ptr = (*datetime).ob_type;
    Py_DECREF(datetime);
    ptr
}

unsafe fn look_up_date_type() -> *mut PyTypeObject {
    let date = (PyDateTimeAPI.Date_FromDate)(1970, 1, 1, PyDateTimeAPI.DateType);
    let ptr = (*date).ob_type;
    Py_DECREF(date);
    ptr
}

unsafe fn look_up_time_type() -> *mut PyTypeObject {
    let time = (PyDateTimeAPI.Time_FromTime)(0, 0, 0, 0, NONE, PyDateTimeAPI.TimeType);
    let ptr = (*time).ob_type;
    Py_DECREF(time);
    ptr
}
