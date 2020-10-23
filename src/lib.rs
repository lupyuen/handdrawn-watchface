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
    ffi::c_void,
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
};

///////////////////////////////////////////////////////////////////////////////
//  Watch Face Definition

/// Barebones Watch Face with no frills
pub struct HandDrawnWatchFace {
    /// Image at top left
    top_left_image: lvgl::Ptr,
    /// Image at top right
    top_right_image: lvgl::Ptr,
    /// Image at bottom left
    bottom_left_image: lvgl::Ptr,
    /// Image at bottom right
    bottom_right_image: lvgl::Ptr,
    /// Bitmaps for the 10 digits
    bitmaps: [img::lv_img_dsc_t; 10],
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

        //  Create the widgets
        let watch_face = Self {
            //  Create the top left image
            top_left_image: {
                let image = img::create(screen, ptr::null()) ? ;  //  `?` will terminate the function in case of error
                obj::set_pos(image, 40, 20) ? ;  //  Set image position to top left
                image                            //  Return the image as top_left_image
            },

            //  Create the top right image
            top_right_image: {
                let image = img::create(screen, ptr::null()) ? ;
                obj::set_pos(image, 120, 20) ? ;  //  Set image position to top right
                image                             //  Return the image as top_right_image
            },

            //  Create the bottom left image
            bottom_left_image: {
                let image = img::create(screen, ptr::null()) ? ;
                obj::set_pos(image, 40, 120) ? ;  //  Set image position to bottom left
                image                             //  Return the image as bottom_left_image
            },

            //  Create the bottom right image
            bottom_right_image: {
                let image = img::create(screen, ptr::null()) ? ;
                obj::set_pos(image, 120, 120) ? ;  //  Set image position to bottom right
                image                              //  Return the image as bottom_right_image
            },

            //  Load the bitmaps
            bitmaps: [
                img::lv_img_dsc_t { data: include_bytes!("../bitmaps/0.bin") as *const u8, header, data_size },
                img::lv_img_dsc_t { data: include_bytes!("../bitmaps/1.bin") as *const u8, header, data_size },
                img::lv_img_dsc_t { data: include_bytes!("../bitmaps/2.bin") as *const u8, header, data_size },
                img::lv_img_dsc_t { data: include_bytes!("../bitmaps/3.bin") as *const u8, header, data_size },
                img::lv_img_dsc_t { data: include_bytes!("../bitmaps/4.bin") as *const u8, header, data_size },
                img::lv_img_dsc_t { data: include_bytes!("../bitmaps/5.bin") as *const u8, header, data_size },
                img::lv_img_dsc_t { data: include_bytes!("../bitmaps/6.bin") as *const u8, header, data_size },
                img::lv_img_dsc_t { data: include_bytes!("../bitmaps/7.bin") as *const u8, header, data_size },
                img::lv_img_dsc_t { data: include_bytes!("../bitmaps/8.bin") as *const u8, header, data_size },
                img::lv_img_dsc_t { data: include_bytes!("../bitmaps/9.bin") as *const u8, header, data_size },
            ]
        };
        //  Return the watch face
        Ok(watch_face)
    }

    ///////////////////////////////////////////////////////////////////////////////
    //  Update Watch Face

    /// Update the widgets in the Watch Face with the current time
    fn update(&mut self, state: &WatchFaceState) -> MynewtResult<()> {
        //  Update the top left image with the first digit of the hour
        let digit = state.time.hour / 10;             //  Compute the first digit of the hour
        let bitmap: *mut img::lv_img_dsc_t =          //  Fetch the bitmap for the digit...
            &mut self.bitmaps[digit as usize];        //  As a mutable reference
        img::set_src(                                 //  Set the source...
            self.top_left_image,                      //  Of the the top left image...
            bitmap as *const c_void                   //  To the bitmap digit
        ) ? ;
        
        //  Update the top right image with the second digit of the hour
        let digit = state.time.hour % 10;             //  Compute the second digit of the hour
        let bitmap: *mut img::lv_img_dsc_t =          //  Fetch the bitmap for the digit...
            &mut self.bitmaps[digit as usize];        //  As a mutable reference
        img::set_src(                                 //  Set the source...
            self.top_right_image,                     //  Of the the top right image...
            bitmap as *const c_void                   //  To the bitmap digit
        ) ? ;

        //  Update the bottom left image with the first digit of the minute
        let digit = state.time.minute / 10;           //  Compute the first digit of the minute
        let bitmap: *mut img::lv_img_dsc_t =          //  Fetch the bitmap for the digit...
            &mut self.bitmaps[digit as usize];        //  As a mutable reference
        img::set_src(                                 //  Set the source...
            self.bottom_left_image,                   //  Of the the bottom left image...
            bitmap as *const c_void                   //  To the bitmap digit
        ) ? ;

        //  Update the bottom right image with the second digit of the minute
        let digit = state.time.minute % 10;           //  Compute the second digit of the minute
        let bitmap: *mut img::lv_img_dsc_t =          //  Fetch the bitmap for the digit...
            &mut self.bitmaps[digit as usize];        //  As a mutable reference
        img::set_src(                                 //  Set the source...
            self.bottom_right_image,                  //  Of the the bottom right image...
            bitmap as *const c_void                   //  To the bitmap digit
        ) ? ;

        //  Return OK
        Ok(())
    }
}