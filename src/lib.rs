/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
//! Hand-Drawn Watch Face for Rust + Mynewt on PineTime Smart Watch
#![no_std]                              //  Don't link with standard Rust library, which is not compatible with embedded systems
#![feature(trace_macros)]               //  Allow macro tracing: `trace_macros!(true)`
#![feature(concat_idents)]              //  Allow `concat_idents!()` macro used in `coap!()` macro
#![feature(proc_macro_hygiene)]         //  Allow Procedural Macros like `run!()`
#![feature(exclusive_range_pattern)]    //  Allow ranges like `0..128` in `match` statements

pub use watchface;  //  Export the Watch Face Framework

use core::{
    ptr,
};
use watchface::lvgl::mynewt::{
    result::*,
};
use watchface::lvgl::{
    self,
    core::obj,
    widgets::img,
};
use watchface::{
    WatchFace,
    WatchFaceState,
    WatchFaceTime,
};

///////////////////////////////////////////////////////////////////////////////
//  Watch Face Definition

/// Barebones Watch Face with no frills
pub struct HandDrawnWatchFace {
    /// Bitmaps for the 10 digits
    bitmaps: [img::lv_img_dsc_t; 4],  //  TODO: Change to 10
    /// Image at top left
    top_left_image: lvgl::Ptr,
    /// Image at top right
    top_right_image: lvgl::Ptr,
    /// Image at bottom left
    bottom_left_image: lvgl::Ptr,
    /// Image at bottom right
    bottom_right_image: lvgl::Ptr,
}

/// Width of each image and bitmap
const IMAGE_WIDTH: u32  = 80;

/// Height of each image and bitmap
const IMAGE_HEIGHT: u32 = 100;

/// 2 bytes per pixel, in RGB565 format
const BYTES_PER_PIXEL: u32 = 2;

impl WatchFace for HandDrawnWatchFace {

    ///////////////////////////////////////////////////////////////////////////////
    //  Create Watch Face

    /// Create the widgets for the Watch Face
    fn new() -> MynewtResult<Self> {
        //  Get the active screen
        let screen = watchface::get_active_screen();

        //  Compose the image header
        let mut header = img::lv_img_header_t::default();
        header.set_cf(img::LV_IMG_CF_TRUE_COLOR);  //  Color Format
        header.set_w(IMAGE_WIDTH);                 //  Width
        header.set_h(IMAGE_HEIGHT);                //  Height

        //  Compute the image size
        let data_size = IMAGE_WIDTH * IMAGE_HEIGHT * BYTES_PER_PIXEL;

        //  Load the bitmaps
        let mut bitmaps = [
            img::lv_img_dsc_t { data: include_bytes!("../bitmaps/0.bin") as *const u8, header, data_size },
            img::lv_img_dsc_t { data: include_bytes!("../bitmaps/1.bin") as *const u8, header, data_size },
            img::lv_img_dsc_t { data: include_bytes!("../bitmaps/2.bin") as *const u8, header, data_size },
            img::lv_img_dsc_t { data: include_bytes!("../bitmaps/3.bin") as *const u8, header, data_size },
        ];

        //  Create the widgets
        let watch_face = Self {
            //  Set the loaded bitmaps
            bitmaps,

            //  Create the top left image
            top_left_image: {
                let image = img::create(screen, ptr::null()) ? ;   //  `?` will terminate the function in case of error
                let bitmap: *mut img::lv_img_dsc_t = &mut bitmaps[0];  //  Fetch bitmap for "0"
                img::set_src(image, bitmap as *const core::ffi::c_void) ? ;  //  Set top left image to "0"
                obj::set_pos(image, 40, 20) ? ;  //  Top left
                image  //  Return the image as top_left_image
            },

            //  Create the top right image
            top_right_image: {
                let image = img::create(screen, ptr::null()) ? ;
                let bitmap: *mut img::lv_img_dsc_t = &mut bitmaps[1];  //  Fetch bitmap for "0"
                img::set_src(image, bitmap as *const core::ffi::c_void) ? ;                //  Set top right image to "1"
                obj::set_pos(image, 120, 20) ? ;  //  Top right
                image  //  Return the image as top_right_image
            },

            //  Create the bottom left image
            bottom_left_image: {
                let image = img::create(screen, ptr::null()) ? ;
                let bitmap: *mut img::lv_img_dsc_t = &mut bitmaps[2];  //  Fetch bitmap for "0"
                img::set_src(image, bitmap as *const core::ffi::c_void) ? ;                //  Set bottom left image to "2"
                obj::set_pos(image, 40, 120) ? ;  //  Bottom left
                image  //  Return the image as bottom_left_image
            },

            //  Create the bottom right image
            bottom_right_image: {
                let image = img::create(screen, ptr::null()) ? ;
                let bitmap: *mut img::lv_img_dsc_t = &mut bitmaps[3];  //  Fetch bitmap for "0"
                img::set_src(image, bitmap as *const core::ffi::c_void) ? ;                //  Set bottom right image to "3"
                obj::set_pos(image, 120, 120) ? ;  //  Bottom right
                image  //  Return the image as bottom_right_image
            },
            
        };
        //  Return the watch face
        Ok(watch_face)
    }

    ///////////////////////////////////////////////////////////////////////////////
    //  Update Watch Face

    /// Update the widgets in the Watch Face with the current time
    fn update(&self, state: &WatchFaceState) -> MynewtResult<()> {
        Ok(())
    }
}

impl HandDrawnWatchFace {

    ///////////////////////////////////////////////////////////////////////////////
    //  Update Watch Face

}
