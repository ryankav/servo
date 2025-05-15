/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
 
use dom_struct::dom_struct; 

use crate::dom::sourcebuffer::SourceBuffer;
use crate::dom::bindings::codegen::Bindings::ManagedSourceBufferBinding::ManagedSourceBufferMethods;

#[dom_struct]
pub(crate) struct ManagedSourceBuffer {
  source_buffer: SourceBuffer,
}

impl ManagedSourceBufferMethods<crate::DomTypeHolder> for ManagedSourceBuffer {
  // https://w3c.github.io/media-source/#dfn-bufferedchange
  event_handler!(bufferdchange, GetOnbufferedchange, SetOnbufferedchange);
}
