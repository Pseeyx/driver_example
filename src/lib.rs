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
    let string = CString::new("Hello World!\n").unwrap();

    unsafe {
        DbgPrint(string.as_ptr());
    }

    driver.DriverUnload = Some(driver_exit);

    let mut driver_config = {
        let wdf_driver_config_size: ULONG;

        #[allow(clippy::cast_possible_truncation)]
        {
            const WDF_DRIVER_CONFIG_SIZE: usize = core::mem::size_of::<WDF_DRIVER_CONFIG>();

            const { assert!(WDF_DRIVER_CONFIG_SIZE <= ULONG::MAX as usize) }

            wdf_driver_config_size = WDF_DRIVER_CONFIG_SIZE as ULONG;
        }

        WDF_DRIVER_CONFIG {
            Size: wdf_driver_config_size,
            EvtDriverDeviceAdd: Some(evt_driver_device_add),
            ..WDF_DRIVER_CONFIG::default()
        }
    };

    let driver_attributes = WDF_NO_OBJECT_ATTRIBUTES;
    let driver_handle_output = WDF_NO_HANDLE.cast::<WDFDRIVER>();

    let wdf_driver_create_ntstatus;
    unsafe {
        wdf_driver_create_ntstatus = call_unsafe_wdf_function_binding!(
            WdfDriverCreate,
            driver as PDRIVER_OBJECT,
            registry_path,
            driver_attributes,
            &mut driver_config,
            driver_handle_output,
        );
    }

    let registry_path: UNICODE_STRING = unsafe { *registry_path };
    let number_of_slice_elements = {
        registry_path.Length as usize / core::mem::size_of_val(&unsafe { *registry_path.Buffer })
    };

    let registry_path = String::from_utf16_lossy(unsafe {
        debug_assert!(
            isize::try_from(number_of_slice_elements * core::mem::size_of::<WCHAR>()).is_ok()
        );
        slice::from_raw_parts(registry_path.Buffer, number_of_slice_elements)
    });

    println!("KMDF Driver Entry Complete! Driver Registry Parameter Key: {registry_path}");

    wdf_driver_create_ntstatus
}

extern "C" fn evt_driver_device_add(
    _driver: WDFDRIVER,
    mut device_init: *mut WDFDEVICE_INIT,
) -> NTSTATUS {
    println!("EvtDriverDeviceAdd Entered!");

    let mut device_handle_output: WDFDEVICE = WDF_NO_HANDLE.cast();

    let ntstatus;
    unsafe {
        ntstatus = call_unsafe_wdf_function_binding!(
            WdfDeviceCreate,
            &mut device_init,
            WDF_NO_OBJECT_ATTRIBUTES,
            &mut device_handle_output,
        );
    }

    println!("WdfDeviceCreate NTSTATUS: {ntstatus:#02x}");
    ntstatus
}

extern "C" fn driver_exit(_driver: *mut DRIVER_OBJECT) {
    println!("Goodbye World!");
    println!("Driver Exit Complete!");
}
