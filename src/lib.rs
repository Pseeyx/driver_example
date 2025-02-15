#![no_std]

extern crate alloc;
#[cfg(not(test))]
extern crate wdk_panic;

use alloc::{ffi::CString, slice, string::String};
use wdk::println;
#[cfg(not(test))]
use wdk_alloc::WdkAllocator;
use wdk_sys::{
    call_unsafe_wdf_function_binding, ntddk::DbgPrint, DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING,
    PDRIVER_OBJECT, ULONG, UNICODE_STRING, WCHAR, WDFDEVICE, WDFDEVICE_INIT, WDFDRIVER,
    WDF_DRIVER_CONFIG, WDF_NO_HANDLE, WDF_NO_OBJECT_ATTRIBUTES,
};

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

#[export_name = "DriverEntry"]
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    DbgPrint(CString::new("Hello World!\n").unwrap().as_ptr());
    driver.DriverUnload = Some(driver_exit);

    let mut driver_config = WDF_DRIVER_CONFIG {
        Size: core::mem::size_of::<WDF_DRIVER_CONFIG>() as ULONG,
        EvtDriverDeviceAdd: Some(evt_driver_device_add),
        ..WDF_DRIVER_CONFIG::default()
    };

    let wdf_driver_create_ntstatus = call_unsafe_wdf_function_binding!(
        WdfDriverCreate,
        driver as PDRIVER_OBJECT,
        registry_path,
        WDF_NO_OBJECT_ATTRIBUTES,
        &mut driver_config,
        WDF_NO_HANDLE.cast::<WDFDRIVER>(),
    );

    let registry_path: UNICODE_STRING = *registry_path;
    let registry_path = String::from_utf16_lossy(slice::from_raw_parts(
        registry_path.Buffer,
        registry_path.Length as usize / size_of::<WCHAR>(),
    ));
    wdf_driver_create_ntstatus
}

extern "C" fn evt_driver_device_add(
    _driver: WDFDRIVER,
    mut device_init: *mut WDFDEVICE_INIT,
) -> NTSTATUS {
    println!("EvtDriverDeviceAdd Entered!");

    let ntstatus = call_unsafe_wdf_function_binding!(
        WdfDeviceCreate,
        &mut device_init,
        WDF_NO_OBJECT_ATTRIBUTES,
        WDF_NO_HANDLE.cast::<WDFDEVICE>(),
    );

    println!("WdfDeviceCreate NTSTATUS: {ntstatus:#02x}");
    ntstatus
}

extern "C" fn driver_exit(_driver: *mut DRIVER_OBJECT) {
    println!("Goodbye World!");
    println!("Driver Exit Complete!");
}
