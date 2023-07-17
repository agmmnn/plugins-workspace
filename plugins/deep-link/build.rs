// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::process::exit;

fn main() {
    if let Err(error) = tauri_build::mobile::PluginBuilder::new()
        .android_path("android")
        .run()
    {
        println!("{error:#}");
        exit(1);
    }
}