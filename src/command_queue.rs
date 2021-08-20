// Copyright (c) 2020-2021 Via Technology Ltd. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! OpenCL Command Queue API.

#![allow(non_camel_case_types)]

pub use cl_sys::{
    CL_QUEUE_ON_DEVICE, CL_QUEUE_ON_DEVICE_DEFAULT, CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE,
    CL_QUEUE_PROFILING_ENABLE,
};

use super::error_codes::{CL_INVALID_VALUE, CL_SUCCESS};
use super::info_type::InfoType;
use super::types::{
    cl_bool, cl_command_queue, cl_command_queue_info, cl_command_queue_properties, cl_context,
    cl_device_id, cl_event, cl_int, cl_kernel, cl_map_flags, cl_mem, cl_uint, cl_ulong,
};
#[cfg(feature = "CL_VERSION_1_2")]
use super::types::cl_mem_migration_flags;
#[cfg(feature = "CL_VERSION_2_0")]
use super::types::cl_queue_properties;
#[cfg(feature = "CL_VERSION_2_1")]
use super::types::cl_mem_migration_flags;
use super::{api_info_size, api_info_value, api_info_vector};
use cl_sys::{
    clCreateCommandQueue, clEnqueueCopyBuffer, clEnqueueCopyBufferRect, clEnqueueCopyBufferToImage,
    clEnqueueCopyImage, clEnqueueCopyImageToBuffer, clEnqueueMapBuffer, clEnqueueMapImage,
    clEnqueueNDRangeKernel, clEnqueueNativeKernel, clEnqueueReadBuffer, clEnqueueReadBufferRect,
    clEnqueueReadImage, clEnqueueUnmapMemObject, clEnqueueWriteBuffer, clEnqueueWriteBufferRect,
    clEnqueueWriteImage, clFinish, clFlush, clGetCommandQueueInfo, clReleaseCommandQueue,
    clRetainCommandQueue,
};
#[cfg(feature = "CL_VERSION_1_2")]
use cl_sys::{
    clEnqueueBarrierWithWaitList, clEnqueueFillBuffer, clEnqueueFillImage,
    clEnqueueMarkerWithWaitList, clEnqueueMigrateMemObjects, clEnqueueTask,
};
#[cfg(feature = "CL_VERSION_2_0")]
use cl_sys::{
   clCreateCommandQueueWithProperties, clEnqueueSVMFree, clEnqueueSVMMap, clEnqueueSVMMemFill,
   clEnqueueSVMMemcpy, clEnqueueSVMUnmap,
};
#[cfg(feature = "CL_VERSION_2_1")]
use cl_sys::clEnqueueSVMMigrateMem;

use libc::{c_void, intptr_t, size_t};
use std::mem;
use std::ptr;

/// Create an OpenCL host or device command-queue on a specific device.  
/// Calls clCreateCommandQueue to create an OpenCL context.  
/// Deprecated in CL_VERSION_2_0 by create_command_queue_with_properties.
///
/// * `context` - a valid OpenCL context.
/// * `device` - a device or sub-device associated with context.
/// * `properties` - a list of properties for the command-queue, see
/// [cl_command_queue_properties](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#legacy-queue-properties-table).
///
/// returns a Result containing the new OpenCL command-queue
/// or the error code from the OpenCL C API function.
#[inline]
pub fn create_command_queue(
    context: cl_context,
    device: cl_device_id,
    properties: cl_command_queue_properties,
) -> Result<cl_command_queue, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let queue: cl_command_queue =
        unsafe { clCreateCommandQueue(context, device, properties, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(queue)
    }
}

/// Create an OpenCL host or device command-queue on a specific device.  
/// Calls clCreateCommandQueueWithProperties to create an OpenCL context.  
/// CL_VERSION_2_0 onwards.
///
/// * `context` - a valid OpenCL context.
/// * `device` - a device or sub-device associated with context.
/// * `properties` - a null terminated list of properties for the command-queue, see
/// [cl_queue_properties](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#queue-properties-table).
///
/// returns a Result containing the new OpenCL command-queue
/// or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_2_0")]
#[inline]
pub fn create_command_queue_with_properties(
    context: cl_context,
    device: cl_device_id,
    properties: *const cl_queue_properties,
) -> Result<cl_command_queue, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let queue: cl_command_queue =
        unsafe { clCreateCommandQueueWithProperties(context, device, properties, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(queue)
    }
}

/// Retain an OpenCL command-queue.  
/// Calls clRetainCommandQueue to increment the command-queue reference count.
///
/// * `command_queue` - the OpenCL command-queue.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn retain_command_queue(command_queue: cl_command_queue) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clRetainCommandQueue(command_queue) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Release an OpenCL command-queue.  
/// Calls clReleaseCommandQueue to decrement the command-queue reference count.
///
///  * `command_queue` - the OpenCL command-queue.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn release_command_queue(command_queue: cl_command_queue) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clReleaseCommandQueue(command_queue) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Get data about an OpenCL command-queue.
/// Calls clGetCommandQueueInfo to get the desired data about the command-queue.
pub fn get_command_queue_data(
    command_queue: cl_command_queue,
    param_name: cl_command_queue_info,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetCommandQueueInfo);
    let size = get_size(command_queue, param_name)?;
    api_info_vector!(get_vector, u8, clGetCommandQueueInfo);
    Ok(get_vector(command_queue, param_name, size)?)
}

// cl_command_queue_info
#[derive(Clone, Copy, Debug)]
pub enum CommandQueueInfo {
    CL_QUEUE_CONTEXT = 0x1090,
    CL_QUEUE_DEVICE = 0x1091,
    CL_QUEUE_REFERENCE_COUNT = 0x1092,
    CL_QUEUE_PROPERTIES = 0x1093,
    // CL_VERSION_2_0
    CL_QUEUE_SIZE = 0x1094,
    // CL_VERSION_2_1
    CL_QUEUE_DEVICE_DEFAULT = 0x1095,
    // CL_VERSION_3_0
    CL_QUEUE_PROPERTIES_ARRAY = 0x1098,
}

/// Get specific information about an OpenCL command-queue.  
/// Calls clGetCommandQueueInfo to get the desired information about the command-queue.
///
/// * `command_queue` - the OpenCL command-queue.
/// * `param_name` - the type of command-queue information being queried, see:
/// [Command Queue Parameter](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#command-queue-param-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_command_queue_info(
    command_queue: cl_command_queue,
    param_name: CommandQueueInfo,
) -> Result<InfoType, cl_int> {
    let param_id = param_name as cl_command_queue_info;
    match param_name {
        CommandQueueInfo::CL_QUEUE_REFERENCE_COUNT
        | CommandQueueInfo::CL_QUEUE_SIZE // CL_VERSION_2_0
         => {
            api_info_value!(get_value, cl_uint, clGetCommandQueueInfo);
            Ok(InfoType::Uint(get_value(command_queue, param_id)?))
        }

        CommandQueueInfo::CL_QUEUE_PROPERTIES => {
            api_info_value!(get_value, cl_ulong, clGetCommandQueueInfo);
            Ok(InfoType::Ulong(get_value(command_queue, param_id)?))
        }

        CommandQueueInfo::CL_QUEUE_CONTEXT
        | CommandQueueInfo::CL_QUEUE_DEVICE
        | CommandQueueInfo::CL_QUEUE_DEVICE_DEFAULT // CL_VERSION_2_1
        => {
            api_info_value!(get_value, intptr_t, clGetCommandQueueInfo);
            Ok(InfoType::Ptr(get_value(command_queue, param_id)?))
        }

        CommandQueueInfo::CL_QUEUE_PROPERTIES_ARRAY // CL_VERSION_3_0
        => {
            api_info_size!(get_size, clGetCommandQueueInfo);
            api_info_vector!(get_vec, cl_ulong, clGetCommandQueueInfo);
            let size = get_size(command_queue, param_id)?;
            Ok(InfoType::VecUlong(get_vec(
                command_queue,
                param_id,
                size,
            )?))
        }
    }
}

/// Flush commands to a device.  
/// Calls clFlush to flush an OpenCL command-queue.  
///
/// * `command_queue` - the OpenCL command-queue.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn flush(command_queue: cl_command_queue) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clFlush(command_queue) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Wait for completion of commands on a device.  
/// Calls clFinish and blocks until all previously queued commands have completed.
///
/// * `command_queue` - the OpenCL command-queue.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn finish(command_queue: cl_command_queue) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clFinish(command_queue) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

// OpenCL command-queue enqueue commands.

#[inline]
pub fn enqueue_read_buffer(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_read: cl_bool,
    offset: size_t,
    size: size_t,
    ptr: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueReadBuffer(
            command_queue,
            buffer,
            blocking_read,
            offset,
            size,
            ptr,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_read_buffer_rect(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_read: cl_bool,
    buffer_origin: *const size_t,
    host_origin: *const size_t,
    region: *const size_t,
    buffer_row_pitch: size_t,
    buffer_slice_pitch: size_t,
    host_row_pitch: size_t,
    host_slice_pitch: size_t,
    ptr: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueReadBufferRect(
            command_queue,
            buffer,
            blocking_read,
            buffer_origin,
            host_origin,
            region,
            buffer_row_pitch,
            buffer_slice_pitch,
            host_row_pitch,
            host_slice_pitch,
            ptr,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_write_buffer(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_write: cl_bool,
    offset: size_t,
    size: size_t,
    ptr: *const c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueWriteBuffer(
            command_queue,
            buffer,
            blocking_write,
            offset,
            size,
            ptr,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_write_buffer_rect(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_write: cl_bool,
    buffer_origin: *const size_t,
    host_origin: *const size_t,
    region: *const size_t,
    buffer_row_pitch: size_t,
    buffer_slice_pitch: size_t,
    host_row_pitch: size_t,
    host_slice_pitch: size_t,
    ptr: *const c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueWriteBufferRect(
            command_queue,
            buffer,
            blocking_write,
            buffer_origin,
            host_origin,
            region,
            buffer_row_pitch,
            buffer_slice_pitch,
            host_row_pitch,
            host_slice_pitch,
            ptr,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
pub fn enqueue_fill_buffer(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    pattern: *const c_void,
    pattern_size: size_t,
    offset: size_t,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueFillBuffer(
            command_queue,
            buffer,
            pattern,
            pattern_size,
            offset,
            size,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_copy_buffer(
    command_queue: cl_command_queue,
    src_buffer: cl_mem,
    dst_buffer: cl_mem,
    src_offset: size_t,
    dst_offset: size_t,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueCopyBuffer(
            command_queue,
            src_buffer,
            dst_buffer,
            src_offset,
            dst_offset,
            size,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_copy_buffer_rect(
    command_queue: cl_command_queue,
    src_buffer: cl_mem,
    dst_buffer: cl_mem,
    src_origin: *const size_t,
    dst_origin: *const size_t,
    region: *const size_t,
    src_row_pitch: size_t,
    src_slice_pitch: size_t,
    dst_row_pitch: size_t,
    dst_slice_pitch: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueCopyBufferRect(
            command_queue,
            src_buffer,
            dst_buffer,
            src_origin,
            dst_origin,
            region,
            src_row_pitch,
            src_slice_pitch,
            dst_row_pitch,
            dst_slice_pitch,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_read_image(
    command_queue: cl_command_queue,
    image: cl_mem,
    blocking_read: cl_bool,
    origin: *const size_t,
    region: *const size_t,
    row_pitch: size_t,
    slice_pitch: size_t,
    ptr: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueReadImage(
            command_queue,
            image,
            blocking_read,
            origin,
            region,
            row_pitch,
            slice_pitch,
            ptr,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_write_image(
    command_queue: cl_command_queue,
    image: cl_mem,
    blocking_write: cl_bool,
    origin: *const size_t,
    region: *const size_t,
    row_pitch: size_t,
    slice_pitch: size_t,
    ptr: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueWriteImage(
            command_queue,
            image,
            blocking_write,
            origin,
            region,
            row_pitch,
            slice_pitch,
            ptr,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
pub fn enqueue_fill_image(
    command_queue: cl_command_queue,
    image: cl_mem,
    fill_color: *const c_void,
    origin: *const size_t,
    region: *const size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueFillImage(
            command_queue,
            image,
            fill_color,
            origin,
            region,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_copy_image(
    command_queue: cl_command_queue,
    src_image: cl_mem,
    dst_image: cl_mem,
    src_origin: *const size_t,
    dst_origin: *const size_t,
    region: *const size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueCopyImage(
            command_queue,
            src_image,
            dst_image,
            src_origin,
            dst_origin,
            region,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_copy_image_to_buffer(
    command_queue: cl_command_queue,
    src_image: cl_mem,
    dst_buffer: cl_mem,
    src_origin: *const size_t,
    region: *const size_t,
    dst_offset: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueCopyImageToBuffer(
            command_queue,
            src_image,
            dst_buffer,
            src_origin,
            region,
            dst_offset,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_copy_buffer_to_image(
    command_queue: cl_command_queue,
    src_buffer: cl_mem,
    dst_image: cl_mem,
    src_offset: size_t,
    dst_origin: *const size_t,
    region: *const size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueCopyBufferToImage(
            command_queue,
            src_buffer,
            dst_image,
            src_offset,
            dst_origin,
            region,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

/// Note: returns event NOT pointer for consistency with other enqueue functions.  
/// The buffer pointer is returned in the buffer_ptr mutable reference.
#[inline]
pub fn enqueue_map_buffer(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_map: cl_bool,
    map_flags: cl_map_flags,
    offset: size_t,
    size: size_t,
    buffer_ptr: &mut cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let mut status: cl_int = CL_INVALID_VALUE;
    *buffer_ptr = unsafe {
        clEnqueueMapBuffer(
            command_queue,
            buffer,
            blocking_map,
            map_flags,
            offset,
            size,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

/// Note: returns event NOT pointer for consistency with other enqueue functions.  
/// The image pointer is returned in the image_ptr mutable reference.
#[inline]
pub fn enqueue_map_image(
    command_queue: cl_command_queue,
    image: cl_mem,
    blocking_map: cl_bool,
    map_flags: cl_map_flags,
    origin: *const size_t,
    region: *const size_t,
    image_row_pitch: *mut size_t,
    image_slice_pitch: *mut size_t,
    image_ptr: &mut cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<*mut c_void, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let mut status: cl_int = CL_INVALID_VALUE;
    *image_ptr = unsafe {
        clEnqueueMapImage(
            command_queue,
            image,
            blocking_map,
            map_flags,
            origin,
            region,
            image_row_pitch,
            image_slice_pitch,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_unmap_mem_object(
    command_queue: cl_command_queue,
    memobj: cl_mem,
    mapped_ptr: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueUnmapMemObject(
            command_queue,
            memobj,
            mapped_ptr,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
pub fn enqueue_migrate_mem_object(
    command_queue: cl_command_queue,
    num_mem_objects: cl_uint,
    mem_objects: *const cl_mem,
    flags: cl_mem_migration_flags,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueMigrateMemObjects(
            command_queue,
            num_mem_objects,
            mem_objects,
            flags,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_nd_range_kernel(
    command_queue: cl_command_queue,
    kernel: cl_kernel,
    work_dim: cl_uint,
    global_work_offset: *const size_t,
    global_work_dims: *const size_t,
    local_work_dims: *const size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueNDRangeKernel(
            command_queue,
            kernel,
            work_dim,
            global_work_offset,
            global_work_dims,
            local_work_dims,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

// Deprecated in CL_VERSION_2_0
#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
pub fn enqueue_task(
    command_queue: cl_command_queue,
    kernel: cl_kernel,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueTask(
            command_queue,
            kernel,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[inline]
pub fn enqueue_native_kernel(
    command_queue: cl_command_queue,
    user_func: Option<extern "C" fn(*mut c_void)>,
    args: *mut c_void,
    cb_args: size_t,
    num_mem_objects: cl_uint,
    mem_list: *const cl_mem,
    args_mem_loc: *const *const c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueNativeKernel(
            command_queue,
            user_func,
            args,
            cb_args,
            num_mem_objects,
            mem_list,
            args_mem_loc,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
pub fn enqueue_marker_with_wait_list(
    command_queue: cl_command_queue,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueMarkerWithWaitList(
            command_queue,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
pub fn enqueue_barrier_with_wait_list(
    command_queue: cl_command_queue,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueBarrierWithWaitList(
            command_queue,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[cfg(feature = "CL_VERSION_2_0")]
#[inline]
pub fn enqueue_svm_free(
    command_queue: cl_command_queue,
    num_svm_pointers: cl_uint,
    svm_pointers: *const *const c_void,
    pfn_free_func: Option<
        extern "C" fn(
            queue: cl_command_queue,
            num_svm_pointers: cl_uint,
            svm_pointers: *const *const c_void,
            user_data: *mut c_void,
        ),
    >,
    user_data: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueSVMFree(
            command_queue,
            num_svm_pointers,
            svm_pointers,
            pfn_free_func,
            user_data,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[cfg(feature = "CL_VERSION_2_0")]
#[inline]
pub fn enqueue_svm_mem_cpy(
    command_queue: cl_command_queue,
    blocking_copy: cl_bool,
    dst_ptr: *mut c_void,
    src_ptr: *const c_void,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueSVMMemcpy(
            command_queue,
            blocking_copy,
            dst_ptr,
            src_ptr,
            size,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[cfg(feature = "CL_VERSION_2_0")]
#[inline]
pub fn enqueue_svm_mem_fill(
    command_queue: cl_command_queue,
    svm_ptr: *mut c_void,
    pattern: *const c_void,
    pattern_size: size_t,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueSVMMemFill(
            command_queue,
            svm_ptr,
            pattern,
            pattern_size,
            size,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[cfg(feature = "CL_VERSION_2_0")]
#[inline]
pub fn enqueue_svm_map(
    command_queue: cl_command_queue,
    blocking_map: cl_bool,
    flags: cl_map_flags,
    svm_ptr: *mut c_void,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueSVMMap(
            command_queue,
            blocking_map,
            flags,
            svm_ptr,
            size,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[cfg(feature = "CL_VERSION_2_0")]
#[inline]
pub fn enqueue_svm_unmap(
    command_queue: cl_command_queue,
    svm_ptr: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueSVMUnmap(
            command_queue,
            svm_ptr,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[cfg(feature = "CL_VERSION_2_1")]
#[inline]
pub fn enqueue_svm_migrate_mem(
    command_queue: cl_command_queue,
    num_svm_pointers: cl_uint,
    svm_pointers: *const *const c_void,
    sizes: *const size_t,
    flags: cl_mem_migration_flags,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueSVMMigrateMem(
            command_queue,
            num_svm_pointers,
            svm_pointers,
            sizes,
            flags,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{create_context, release_context};
    use crate::device::{get_device_ids, CL_DEVICE_TYPE_GPU};
    use crate::platform::get_platform_ids;
    use crate::error_codes::error_text;

    #[test]
    fn test_command_queue() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose the first platform
        let platform_id = platform_ids[0];

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_GPU).unwrap();
        assert!(0 < device_ids.len());

        let device_id = device_ids[0];

        let context = create_context(&device_ids, ptr::null(), None, ptr::null_mut());
        let context = context.unwrap();

        let queue = create_command_queue(context, device_id,
            CL_QUEUE_PROFILING_ENABLE | CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE).unwrap();

        let value = get_command_queue_info(queue, CommandQueueInfo::CL_QUEUE_CONTEXT).unwrap();
        let value = value.to_ptr();
        println!("CL_QUEUE_CONTEXT: {:X}", value);
        assert_eq!(context, value as cl_context);

        let value = get_command_queue_info(queue, CommandQueueInfo::CL_QUEUE_DEVICE).unwrap();
        let value = value.to_ptr();
        println!("CL_QUEUE_DEVICE: {:X}", value);
        assert_eq!(device_id, value as cl_device_id);

        let value = get_command_queue_info(queue, CommandQueueInfo::CL_QUEUE_REFERENCE_COUNT).unwrap();
        let value = value.to_uint();
        println!("CL_QUEUE_REFERENCE_COUNT: {}", value);
        assert_eq!(1, value);

        let value = get_command_queue_info(queue, CommandQueueInfo::CL_QUEUE_PROPERTIES).unwrap();
        let value = value.to_ulong();
        println!("CL_QUEUE_PROPERTIES: {}", value);

        // CL_VERSION_2_0 value
        match get_command_queue_info(queue, CommandQueueInfo::CL_QUEUE_SIZE) {
            Ok(value) => {
                let value = value.to_uint();
                println!("CL_QUEUE_SIZE: {}", value);
            }
            Err(e) => println!("OpenCL error, CL_QUEUE_SIZE: {}", error_text(e))
        };

        // CL_VERSION_2_1 value
        match get_command_queue_info(queue, CommandQueueInfo::CL_QUEUE_DEVICE_DEFAULT) {
            Ok(value) => {
                let value = value.to_ptr();
                println!("CL_QUEUE_DEVICE_DEFAULT: {:X}", value);
            }
            Err(e) => println!("OpenCL error, CL_QUEUE_DEVICE_DEFAULT: {}", error_text(e))
        };
        
        // CL_VERSION_3_0 value
        match get_command_queue_info(queue, CommandQueueInfo::CL_QUEUE_PROPERTIES_ARRAY) {
            Ok(value) => {
                let value = value.to_vec_ulong();
                println!("CL_QUEUE_PROPERTIES_ARRAY: {}", value.len());
            }
            Err(e) => println!("OpenCL error, CL_QUEUE_PROPERTIES_ARRAY: {}", error_text(e))
        };
        
        release_command_queue(queue).unwrap();

        release_context(context).unwrap();
    }
}
