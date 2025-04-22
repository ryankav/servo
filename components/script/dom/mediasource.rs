/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::MediaSourceBinding::{
    EndOfStreamError, MediaSourceMethods, ReadyState,
};
use crate::dom::bindings::num::Finite;
use crate::dom::bindings::reflector::reflect_dom_object;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::eventtarget::EventTarget;
use crate::dom::sourcebuffer::SourceBuffer;
use crate::dom::sourcebufferlist::SourceBufferList;
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct MediaSource {
    eventtarget: EventTarget,
    ready_state: ReadyState,
    source_buffers: DomRefCell<SourceBufferList>,
    active_source_buffers: DomRefCell<SourceBufferList>,
    duration: Finite<f64>,
}

impl MediaSource {
    pub(crate) fn new_inherited(
        ready_state: ReadyState,
        source_buffers: SourceBufferList,
        active_source_buffers: SourceBufferList,
        duration: Finite<f64>,
    ) -> MediaSource {
        Self {
            eventtarget: EventTarget::new_inherited(),
            ready_state,
            source_buffers: DomRefCell::new(source_buffers),
            active_source_buffers: DomRefCell::new(active_source_buffers),
            duration,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        window: &Window,
        ready_state: ReadyState,
        source_buffers: SourceBufferList,
        active_source_buffers: SourceBufferList,
        duration: Finite<f64>,
        can_gc: CanGc,
    ) -> DomRoot<MediaSource> {
        reflect_dom_object(
            Box::new(Self::new_inherited(
                ready_state,
                source_buffers,
                active_source_buffers,
                duration,
            )),
            window,
            can_gc,
        )
    }
}

impl MediaSourceMethods<crate::DomTypeHolder> for MediaSource {
    fn SourceBuffers(&self) {
        todo!()
    }

    fn AddSourceBuffer(&self, src: DOMString) {
        todo!()
    }

    fn RemoveSourceBuffer(&self, buf: SourceBuffer) {
        todo!()
    }

    fn EndOfStream(&self, err: Option<EndOfStreamError>) {
        todo!()
    }

    fn ActiveSourceBuffers(&self) {
        todo!()
    }

    fn ReadyState(&self) -> ReadyState {
        todo!()
    }

    fn Duration(&self) -> f64 {
        todo!()
    }

    fn SetDuration(&self, duration: f64) {
        todo!()
    }

    fn SetLiveSeekableRange(&self, start: f64, end: f64) {
        todo!()
    }

    fn ClearLiveSeekableRange(&self) {
        todo!()
    }

    fn IsTypeSupported() {}

    event_handler!(sourceopen, GetOnsourceopen, SetOnsourceopen);
    event_handler!(sourceended, GetOnsourceended, SetOnsourceended);
    event_handler!(sourceclose, GetOnsourceclose, SetOnsourceclose);
}
