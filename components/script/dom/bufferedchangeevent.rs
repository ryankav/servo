/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;
use stylo_atoms::Atom;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::BufferedChangeEvent::{
    BufferedChangeEventInit, BufferedChangeEventMethods,
};
use crate::dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::reflect_dom_object_with_proto;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::globalscope::GlobalScope;
use crate::dom::timeranges::TimeRanges;
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct BufferedChangeEvent {
    event: Event,
    added_ranges: DomRefCell<Option<TimeRanges>>,
    removed_ranges: DomRefCell<Option<TimeRanges>>,
}

impl BufferedChangeEvent {
    fn new_inherited(
        added_ranges: DomRefCell<Option<TimeRanges>>,
        removed_ranges: DomRefCell<Option<TimeRanges>>,
    ) -> BufferedChangeEvent {
        Self {
            event: Event::new_inherited(),
            added_ranges,
            removed_ranges,
        }
    }

    pub(crate) fn new(
        global: &GlobalScope,
        type_: Atom,
        can_bubble: EventBubbles,
        cancelable: EventCancelable,
        added_ranges: DomRefCell<Option<TimeRanges>>,
        removed_ranges: DomRefCell<Option<TimeRanges>>,
        can_gc: CanGc,
    ) -> DomRoot<BufferedChangeEvent> {
        Self::new_with_proto(
            global,
            None,
            type_,
            can_bubble,
            cancelable,
            added_ranges,
            removed_ranges,
            can_gc,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new_with_proto(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        type_: Atom,
        can_bubble: EventBubbles,
        cancelable: EventCancelable,
        added_ranges: DomRefCell<Option<TimeRanges>>,
        removed_ranges: DomRefCell<Option<TimeRanges>>,
        can_gc: CanGc,
    ) -> DomRoot<BufferedChangeEvent> {
        let ev = reflect_dom_object_with_proto(
            Box::new(Self::new_inherited(added_ranges, removed_ranges)),
            global,
            proto,
            can_gc,
        );
        {
            let event = ev.upcast::<Event>();
            event.init_event(type_, bool::from(can_bubble), bool::from(cancelable));
        }
        ev
    }
}

impl BufferedChangeEventMethods<crate::DomTypeHolder> for BufferedChangeEvent {
    // https://w3c.github.io/media-source/#dom-bufferedchangeevent
    fn Constructor(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        type_: DOMString,
        init: &BufferedChangeEventInit,
    ) -> Fallible<DomRoot<BufferedChangeEvent>> {
        let bubbles = EventBubbles::from(init.parent.bubbles);
        let cancelable = EventCancelable::from(init.parent.cancelable);
        let ev = Self::new_with_proto(
            global,
            proto,
            Atom::from(type_),
            bubbles,
            cancelable,
            init.added_ranges,
            init.removed_ranges,
            can_gc,
        );
        Ok(ev)
    }

    // https://w3c.github.io/media-source/#dom-bufferedchangeevent-addedranges
    fn AddedRanges(&self) -> DomRefCell<Option<TimeRanges>> {
        self.added_ranges.clone()
    }

    // https://w3c.github.io/media-source/#dom-bufferedchangeevent-removedranges
    fn RemovedRanges(&self) -> DomRefCell<Option<TimeRanges>> {
        self.removed_ranges.clone()
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
