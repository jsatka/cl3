// Copyright (c) 2020 Via Technology Ltd. All Rights Reserved.
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

//! OpenCL Platform API.

#![allow(non_camel_case_types)]

use super::error_codes::CL_SUCCESS;
use super::info_type::InfoType;
use super::types::{cl_int, cl_name_version, cl_platform_id, cl_platform_info, cl_uint, cl_ulong};
use super::{api_info_size, api_info_value, api_info_vector};
use cl_sys::{clGetPlatformIDs, clGetPlatformInfo};

use libc::{c_void, size_t};
use std::mem;
use std::ptr;

/// Get the available platforms.  
/// Calls clGetPlatformIDs to get the available platform ids.
///  # Examples
/// ```
/// use cl3::platform::get_platform_ids;
///
/// let platform_ids = get_platform_ids().unwrap();
/// println!("Number of OpenCL platforms: {}", platform_ids.len());
/// assert!(0 < platform_ids.len());
/// ```
/// returns a Result containing a vector of available platform ids
/// or the error code from the OpenCL C API function.
pub fn get_platform_ids() -> Result<Vec<cl_platform_id>, cl_int> {
    // Get the number of platforms
    let mut count: cl_uint = 0;
    let mut status = unsafe { clGetPlatformIDs(0, ptr::null_mut(), &mut count) };

    if CL_SUCCESS != status {
        Err(status)
    } else {
        if 0 < count {
            // Get the platform ids.
            let len = count as usize;
            let mut ids: Vec<cl_platform_id> = Vec::with_capacity(len);
            unsafe {
                ids.set_len(len);
                status = clGetPlatformIDs(count, ids.as_mut_ptr(), ptr::null_mut());
            };

            if CL_SUCCESS != status {
                Err(status)
            } else {
                Ok(ids)
            }
        } else {
            Ok(Vec::default())
        }
    }
}

// cl_platform_info
#[derive(Clone, Copy, Debug)]
pub enum PlatformInfo {
    CL_PLATFORM_PROFILE = 0x0900,
    CL_PLATFORM_VERSION = 0x0901,
    CL_PLATFORM_NAME = 0x0902,
    CL_PLATFORM_VENDOR = 0x0903,
    CL_PLATFORM_EXTENSIONS = 0x0904,
    // CL_VERSION_2_1
    CL_PLATFORM_HOST_TIMER_RESOLUTION = 0x0905,
    // CL_VERSION_3_0
    CL_PLATFORM_NUMERIC_VERSION = 0x0906,
    // CL_VERSION_3_0
    CL_PLATFORM_EXTENSIONS_WITH_VERSION = 0x0907,
}

/// Get specific information about an OpenCL platform.
/// Calls clGetPlatformInfo to get the desired information about the platform.
///  # Examples
/// ```
/// use cl3::platform::{get_platform_ids, get_platform_info, PlatformInfo};
///
/// let platform_ids = get_platform_ids().unwrap();
/// assert!(0 < platform_ids.len());
///
/// // Choose a the first platform
/// let platform_id = platform_ids[0];
///
/// let value = get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_NAME).unwrap();
/// let value = value.to_str().unwrap().into_string().unwrap();
/// println!("CL_PLATFORM_NAME: {}", value);
///
/// assert!(0 < value.len());
///
/// let value = get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_VERSION).unwrap();
/// let value = value.to_str().unwrap().into_string().unwrap();
/// println!("CL_PLATFORM_VERSION: {}", value);
/// assert!(0 < value.len());
/// ```
/// * `platform` - the cl_platform_id of the OpenCL platform.
/// * `param_name` - the type of platform information being queried, see
/// [Platform Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#platform-queries-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_platform_info(
    platform: cl_platform_id,
    param_name: PlatformInfo,
) -> Result<InfoType, cl_int> {
    api_info_size!(get_size, clGetPlatformInfo);

    let param_id = param_name as cl_platform_info;
    match param_name {
        PlatformInfo::CL_PLATFORM_PROFILE
        | PlatformInfo::CL_PLATFORM_VERSION
        | PlatformInfo::CL_PLATFORM_NAME
        | PlatformInfo::CL_PLATFORM_VENDOR
        | PlatformInfo::CL_PLATFORM_EXTENSIONS => {
            api_info_vector!(get_string, u8, clGetPlatformInfo);
            let size = get_size(platform, param_id)?;
            Ok(InfoType::Str(get_string(platform, param_id, size)?))
        }
        // CL_VERSION_3_0
        PlatformInfo::CL_PLATFORM_NUMERIC_VERSION => {
            api_info_value!(get_value, cl_uint, clGetPlatformInfo);
            Ok(InfoType::Uint(get_value(platform, param_id)?))
        }

        // CL_VERSION_2_1
        PlatformInfo::CL_PLATFORM_HOST_TIMER_RESOLUTION => {
            api_info_value!(get_value, cl_ulong, clGetPlatformInfo);
            Ok(InfoType::Ulong(get_value(platform, param_id)?))
        }

        // CL_VERSION_3_0
        PlatformInfo::CL_PLATFORM_EXTENSIONS_WITH_VERSION => {
            api_info_vector!(get_vec, cl_name_version, clGetPlatformInfo);
            let size = get_size(platform, param_id)?;
            Ok(InfoType::VecNameVersion(get_vec(platform, param_id, size)?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_platform_info() {
        let platform_ids = get_platform_ids().unwrap();
        println!("Number of platforms: {}", platform_ids.len());
        assert!(0 < platform_ids.len());

        // Choose the first platform
        let platform_id = platform_ids[0];

        let value = get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_PROFILE).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_PLATFORM_PROFILE: {:?}", value);
        assert!(0 < value.to_bytes().len());

        let value = get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_VERSION).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_PLATFORM_VERSION: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let opencl_2_1: String = "OpenCL 2.1".to_string();
        let is_opencl_2_1: bool = value.contains(&opencl_2_1);

        let value = get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_NAME).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_PLATFORM_NAME: {:?}", value);
        assert!(0 < value.to_bytes().len());

        let value = get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_VENDOR).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_PLATFORM_VENDOR: {:?}", value);
        assert!(0 < value.to_bytes().len());

        let value = get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_EXTENSIONS).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_PLATFORM_EXTENSIONS: {:?}", value);
        assert!(0 < value.to_bytes().len());

        if is_opencl_2_1 {
            let value =
                get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_HOST_TIMER_RESOLUTION)
                    .unwrap();
            let value = value.to_ulong();
            println!("CL_PLATFORM_HOST_TIMER_RESOLUTION: {}", value);
            assert!(0 < value);
        }
    }

    #[test]
    fn test_get_platform_info_3_0() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose the first platform
        let platform_id = platform_ids[0];

        let value = get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_VERSION).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_PLATFORM_VERSION: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let opencl_3: String = "OpenCL 3".to_string();
        let is_opencl_3: bool = value.contains(&opencl_3);

        if is_opencl_3 {
            let value =
                get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_NUMERIC_VERSION).unwrap();
            let value = value.to_uint();
            println!("CL_PLATFORM_NUMERIC_VERSION: {}", value);
            assert!(0 < value);

            let value = get_platform_info(
                platform_id,
                PlatformInfo::CL_PLATFORM_EXTENSIONS_WITH_VERSION,
            )
            .unwrap();
            let value = value.to_vec_name_version();
            println!("CL_PLATFORM_EXTENSIONS_WITH_VERSION: {}", value.len());
            println!("CL_PLATFORM_EXTENSIONS_WITH_VERSION: {:?}", value);
            assert!(0 < value.len());
        }
    }
}
