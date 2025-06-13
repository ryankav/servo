/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;
use stylo_atoms::Atom;

use crate::dom::bindings::codegen::Bindings::BufferedChangeEventBinding::{
    BufferedChangeEventInit, BufferedChangeEventMethods,
};
use crate::dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::reflect_dom_object_with_proto;
use crate::dom::bindings::root::{Dom, DomRoot};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::timeranges::{TimeRanges, TimeRangesContainer};
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct BufferedChangeEvent {
    event: Event,
    added_ranges: Dom<TimeRanges>,
    removed_ranges: Dom<TimeRanges>,
}

impl BufferedChangeEvent {
    fn new_inherited(
        added_ranges: &TimeRanges,
        removed_ranges: &TimeRanges,
    ) -> BufferedChangeEvent {
        BufferedChangeEvent {
            event: Event::new_inherited(),
            added_ranges: Dom::from_ref(added_ranges),
            removed_ranges: Dom::from_ref(removed_ranges),
        }
    }

    fn new(
        window: &Window,
        type_: Atom,
        added_ranges: &TimeRanges,
        removed_ranges: &TimeRanges,
        can_gc: CanGc,
    ) -> DomRoot<BufferedChangeEvent> {
        BufferedChangeEvent::new_with_proto(
            window,
            None,
            type_,
            EventBubbles::from(false),
            EventCancelable::from(false),
            added_ranges,
            removed_ranges,
            can_gc,
        )
    }

    fn new_with_proto(
        window: &Window,
        proto: Option<HandleObject>,
        type_: Atom,
        bubbles: EventBubbles,
        cancelable: EventCancelable,
        added_ranges: &TimeRanges,
        removed_ranges: &TimeRanges,
        can_gc: CanGc,
    ) -> DomRoot<BufferedChangeEvent> {
        let ev = reflect_dom_object_with_proto(
            Box::new(BufferedChangeEvent::new_inherited(
                added_ranges,
                removed_ranges,
            )),
            window,
            proto,
            can_gc,
        );

        {
            let event = ev.upcast::<Event>();
            event.init_event(type_, bool::from(bubbles), bool::from(cancelable));
        }
        ev
    }
}

impl BufferedChangeEventMethods<crate::DomTypeHolder> for BufferedChangeEvent {
    // https://w3c.github.io/media-source/#dom-bufferedchangeevent
    fn Constructor(
        window: &Window,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        type_: DOMString,
        init: &BufferedChangeEventInit,
    ) -> DomRoot<BufferedChangeEvent> {
        let bubbles = EventBubbles::from(init.parent.bubbles);
        let cancelable = EventCancelable::from(init.parent.cancelable);

        BufferedChangeEvent::new_with_proto(
            window,
            proto,
            Atom::from(type_),
            bubbles,
            cancelable,
            &init.addedRanges.clone().unwrap_or(TimeRanges::new(
                window,
                TimeRangesContainer::default(),
                CanGc::note(),
            )),
            &init.removedRanges.clone().unwrap_or(TimeRanges::new(
                window,
                TimeRangesContainer::default(),
                CanGc::note(),
            )),
            can_gc,
        )
    }

    // https://w3c.github.io/media-source/#dom-bufferedchangeevent-addedranges
    fn AddedRanges(&self) -> DomRoot<TimeRanges> {
        DomRoot::from_ref(&*self.added_ranges)
    }

    // https://w3c.github.io/media-source/#dom-bufferedchangeevent-removedranges
    fn RemovedRanges(&self) -> DomRoot<TimeRanges> {
        DomRoot::from_ref(&*self.removed_ranges)
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
