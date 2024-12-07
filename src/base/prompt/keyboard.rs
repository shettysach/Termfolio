use crate::commands::autocomplete;
use leptos::{
    ev::{keydown, KeyboardEvent},
    html::Input,
    NodeRef, ReadSignal, SignalGet, SignalUpdate, WriteSignal,
};
use leptos_use::use_event_listener;
use std::{cmp::Ordering, collections::VecDeque};

pub fn keyboard_commands(
    input_element: NodeRef<Input>,
    history: ReadSignal<VecDeque<String>>,
    history_index: ReadSignal<u8>,
    set_history_index: WriteSignal<u8>,
    submitter: WriteSignal<u8>,
) {
    let _ = use_event_listener(input_element, keydown, move |ev: KeyboardEvent| {
        let hist = history.get();
        let index = history_index.get().into();
        let inp = input_element.get().unwrap();

        match &ev.key()[..] {
            //Previous command in history
            "ArrowUp" => {
                ev.prevent_default();
                if index < hist.len() {
                    inp.set_value(&hist[index]);
                    set_history_index.update(move |history_index| *history_index += 1);
                }
            }

            //Next command in history
            "ArrowDown" => match index.cmp(&1) {
                Ordering::Greater => {
                    inp.set_value(&hist[index - 2]);
                    set_history_index.update(move |history_index| *history_index -= 1);
                }
                Ordering::Equal => {
                    inp.set_value("");
                    set_history_index.update(move |history_index| *history_index -= 1);
                }
                Ordering::Less => (),
            },

            //Autocomplete
            "Tab" => {
                ev.prevent_default();
                inp.set_value(autocomplete(&inp.value()));
            }
            _ => {}
        }

        //Ctrl
        if ev.ctrl_key() || ev.meta_key() {
            // Clear
            match &ev.key()[..] {
                "l" | "L" => {
                    ev.prevent_default();
                    submitter.update(|prompts| {
                        *prompts = 0;
                    });
                    submitter.update(|prompts| {
                        *prompts += 1;
                    });
                }
                // Can add Ctrl + P / N for history,
                // but will interfere with new window shortcut
                _ => {}
            }
        }
    });
}
