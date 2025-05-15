/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::BufferedChangeEventBinding::{
    BufferedChangeEventInit, BufferedChangeEventMethods,
};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::event::Event;
use crate::dom::timeranges::TimeRanges;
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct BufferedChangeEvent {
    event: Event,
    added_ranges: DomRefCell<Option<TimeRanges>>,
    removed_ranges: DomRefCell<Option<TimeRanges>>,
}

impl BufferedChangeEventMethods<crate::DomTypeHolder> for BufferedChangeEvent {
    // https://w3c.github.io/media-source/#dom-bufferedchangeevent
    fn Constructor(
        _window: &Window,
        _proto: Option<HandleObject>,
        _can_gc: CanGc,
        _type_: DOMString,
        _init: &BufferedChangeEventInit,
    ) -> DomRoot<BufferedChangeEvent> {
        todo!()
    }

    // https://w3c.github.io/media-source/#dom-bufferedchangeevent-addedranges
    fn AddedRanges(&self) -> DomRoot<TimeRanges> {
        todo!()
    }

    // https://w3c.github.io/media-source/#dom-bufferedchangeevent-removedranges
    fn RemovedRanges(&self) -> DomRoot<TimeRanges> {
        todo!()
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        todo!()
    }
}
