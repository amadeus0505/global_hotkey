// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use std::collections::HashMap;

pub fn start() {
    let event_loop = EventLoopBuilder::new().build().unwrap();

    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();

    let mut hotkeys = HashMap::new();
    hotkeys.insert("F2", HotKey::new(None, Code::F2));
    hotkeys.insert("F3", HotKey::new(None, Code::F3));
    hotkeys.insert("F4", HotKey::new(None, Code::F4));
    hotkeys.insert("CM", HotKey::new(None, Code::KeyL));

    // for hotkey in hotkeys.values() {
    //     match hotkeys_manager.register(*hotkey) {
    //         Ok(()) => println!("Hotkey {:?} added", hotkey),
    //         Err(e) => println!("Adding failed: {}", e)
    //     }
    // }
    
    let global_hotkey_channel = GlobalHotKeyEvent::receiver();
    let mut modifier = false;

    event_loop
        .run(move |_event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);

            if let Ok(event) = global_hotkey_channel.try_recv() {
                // Update modifier
                if event.id() == hotkeys["CM"].id() && event.state() == HotKeyState::Pressed {
                    println!("Pressed");
                    modifier = true;
                }
                if event.id() == hotkeys["CM"].id() && event.state() == HotKeyState::Released {
                    println!("Released");
                    modifier = false;
                }
                // F2 with pressed modifier
                if  event.id() == hotkeys["F2"].id() && 
                    event.state() == HotKeyState::Released &&
                    modifier 
                {
                    println!("Hotkey F2 released");
                }
                // F3 with pressed modifier
                if  event.id() == hotkeys["F3"].id() && 
                    event.state() == HotKeyState::Released &&
                    modifier
                {
                    println!("Hotkey F3 released");
                }
                // F4 pressed with modifier
                if  event.id() == hotkeys["F4"].id() && 
                    event.state() == HotKeyState::Released &&
                    modifier
                {
                    println!("Hotkey F4 released");
                }

                // F2 without modifier
                if  event.id() == hotkeys["F2"].id() && 
                    event.state() == HotKeyState::Released &&
                    !modifier
                {
                    println!("Hotkey F22 released");
                }
                // F3 without modifier
                if  event.id() == hotkeys["F3"].id() && 
                    event.state() == HotKeyState::Released &&
                    !modifier
                {
                    println!("Hotkey F23 released");
                }
                // F4 without modifier
                if  event.id() == hotkeys["F4"].id() && 
                    event.state() == HotKeyState::Released &&
                    !modifier
                {
                    println!("Hotkey F24 released");
                }

                
            }
        })
        .unwrap();
}