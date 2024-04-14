// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use std::collections::HashMap;
use enigo::*;

pub fn start() {
    let mut enigo = Enigo::new();
    let event_loop = EventLoopBuilder::new().build().unwrap();

    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();

    let mut hotkeys = HashMap::new();
    hotkeys.insert("F2", HotKey::new(None, Code::F2));
    hotkeys.insert("F3", HotKey::new(None, Code::F3));
    hotkeys.insert("F4", HotKey::new(None, Code::F4));

    let toggle_key = HotKey::new(None, Code::KeyL);
    match hotkeys_manager.register(toggle_key) {
        Ok(()) => println!("Hotkey {:?} added", toggle_key),
        Err(e) => println!("Adding failed: {}", e)
    }

    for hotkey in hotkeys.values() {
        match hotkeys_manager.register(*hotkey) {
            Ok(()) => println!("Hotkey {:?} added", hotkey),
            Err(e) => println!("Adding failed: {}", e)
        }
    }
    // println!("{} {}", hotkeys["F2"].id(), hotkeys["F22"].id());

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();
    // let mut modifier = false;

    event_loop
        .run(move |_event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);

            if let Ok(event) = global_hotkey_channel.try_recv() {
                if  event.id() == hotkeys["F2"].id() && 
                    event.state() == HotKeyState::Released
                {
                    enigo.key_click(Key::F22);
                }

                if  event.id() == hotkeys["F3"].id() && 
                    event.state() == HotKeyState::Released
                {
                    enigo.key_click(Key::F23);
                }

                if  event.id() == hotkeys["F4"].id() && 
                    event.state() == HotKeyState::Released
                {
                    enigo.key_click(Key::F24);
                }
                if  event.id() == toggle_key.id()
                {
                    println!("Toggle");
                    // cloned() clones every item inside the iterator (hotkeys.values()) -> we get HotKey and not &HotKey
                    // then the cloned values get collected into a vector (this is so the vlaues are stored behind each other)
                    // so the can get turned into a slice (which the function expects)
                    match event.state() {
                        HotKeyState::Pressed => {
                            match hotkeys_manager.unregister_all(hotkeys.values().cloned().collect::<Vec<HotKey>>().as_slice()) {
                                Ok(()) => (),
                                Err(e) => println!("Removing failed: {}", e)
                              }
                        }
                        HotKeyState::Released => {
                            match hotkeys_manager.register_all(hotkeys.values().cloned().collect::<Vec<HotKey>>().as_slice()) {
                                Ok(()) => (),
                                Err(e) => println!("Adding failed: {}", e)
                              }
                        }
                    }
                    
                }
            }
        })
        .unwrap();
}