fn main() {
    unsafe {
        let obj = cjson_rs::cJSON_CreateObject();

        add_field(obj, "hello", "world");
        add_field(obj, "age", "30");
        add_field(obj, "city", "Moscow");

        let json_str = cjson_rs::cJSON_Print(obj);
        let json_rs_str = std::ffi::CStr::from_ptr(json_str).to_str().unwrap();
        println!("{json_rs_str}");
    }
}

unsafe fn add_field(obj: *mut cjson_rs::cJSON, key: &str, value: &str) {
    let key = std::ffi::CString::new(key).unwrap();
    let val = std::ffi::CString::new(value).unwrap();

    unsafe {
        cjson_rs::cJSON_AddStringToObject(obj, key.as_ptr(), val.as_ptr());
    }
}
