/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct; 

use crate::dom::eventtarget::EventTarget;
use crate::dom::bindings::root::DomRoot;
use crate::dom::sourcebuffer::SourceBuffer;
use crate::dom::bindings::codegen::Bindings::SourceBufferListBinding::SourceBufferListMethods;

#[dom_struct]
pub(crate) struct SourceBufferList {
  eventtarget: EventTarget,
}

impl SourceBufferListMethods<crate::DomTypeHolder> for SourceBufferList {
  fn Length(&self) -> u32 {
      todo!()
  }

  fn IndexedGetter(&self, _index: u32) -> Option<DomRoot<SourceBuffer>> {
    todo!()
  }

  event_handler!(addsourcebuffer, GetOnaddsourcebuffer, SetOnaddsourcebuffer);
  event_handler!(removesourcebuffer, GetOnremovesourcebuffer, SetOnremovesourcebuffer);
}