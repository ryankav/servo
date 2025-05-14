/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;

use js::rust::HandleObject;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::MediaSourceBinding::{
    EndOfStreamError, MediaSourceMethods, ReadyState,
};
use crate::dom::bindings::num::Finite;
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

impl MediaSourceMethods<crate::DomTypeHolder> for MediaSource {
    // https://w3c.github.io/media-source/#dom-mediasource-constructor
    fn Constructor(_window: &Window, _handle: Option<HandleObject>, _can_gc: CanGc) -> DomRoot<MediaSource> {
        todo!()
    }

    // https://w3c.github.io/media-source/#dom-mediasource-sourcebuffers 
    fn SourceBuffers(&self) -> DomRoot<SourceBufferList> {
        todo!()
    }

    // https://w3c.github.io/media-source/#activesourcebuffers-attribute
    fn ActiveSourceBuffers(&self) -> DomRoot<SourceBufferList> {
       todo!() 
    }
    
    // https://w3c.github.io/media-source/#readystate-attribute
    fn ReadyState(&self) -> ReadyState {
        todo!()
    }

    // https://w3c.github.io/media-source/#duration-attribute
    fn Duration(&self) -> f64 {
        todo!()
    }

    // https://w3c.github.io/media-source/#duration-attribute
    fn SetDuration(&self, _duration: f64) {
        todo!()
    }

    // https://w3c.github.io/media-source/#addsourcebuffer-method
    fn AddSourceBuffer(&self, _src: DOMString) -> DomRoot<SourceBuffer> {
        todo!()
    }

    // https://w3c.github.io/media-source/#removesourcebuffer-method
    fn RemoveSourceBuffer(&self, _buf: &SourceBuffer) {
        todo!()
    }

    // https://w3c.github.io/media-source/#endofstream-method
    fn EndOfStream(&self, _err: Option<EndOfStreamError>) {
        todo!()
    }

    // https://w3c.github.io/media-source/#setliveseekablerange-method
    fn SetLiveSeekableRange(&self, _start: Finite<f64>, _end: Finite<f64>) {
        todo!()
    }

    // https://w3c.github.io/media-source/#clearliveseekablerange-method
    fn ClearLiveSeekableRange(&self) {
        todo!()
    }

    // https://w3c.github.io/media-source/#istypesupported-method
    fn IsTypeSupported(_window: &Window, _type: DOMString) -> bool {
        todo!()
    }

    // https://w3c.github.io/media-source/#dfn-sourceopen
    event_handler!(sourceopen, GetOnsourceopen, SetOnsourceopen);

    // https://w3c.github.io/media-source/#dfn-sourceended
    event_handler!(sourceended, GetOnsourceended, SetOnsourceended);

    // https://w3c.github.io/media-source/#dfn-sourceclose
    event_handler!(sourceclose, GetOnsourceclose, SetOnsourceclose);
}
