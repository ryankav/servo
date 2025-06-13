/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::SourceBufferListBinding::SourceBufferListMethods;
use crate::dom::bindings::root::DomRoot;
use crate::dom::eventtarget::EventTarget;
use crate::dom::sourcebuffer::SourceBuffer;

#[dom_struct]
pub(crate) struct SourceBufferList {
    eventtarget: EventTarget,
}

impl SourceBufferListMethods<crate::DomTypeHolder> for SourceBufferList {
    // https://w3c.github.io/media-source/#dom-sourcebufferlist-length
    fn Length(&self) -> u32 {
        todo!()
    }

    // https://w3c.github.io/media-source/#dfn-sourcebufferlist-getter
    fn IndexedGetter(&self, _index: u32) -> Option<DomRoot<SourceBuffer>> {
        todo!()
    }

    // https://w3c.github.io/media-source/#dom-sourcebufferlist-onaddsourcebuffer
    event_handler!(addsourcebuffer, GetOnaddsourcebuffer, SetOnaddsourcebuffer);

    // https://w3c.github.io/media-source/#dom-sourcebufferlist-onremovesourcebuffer
    event_handler!(
        removesourcebuffer,
        GetOnremovesourcebuffer,
        SetOnremovesourcebuffer
    );
}
