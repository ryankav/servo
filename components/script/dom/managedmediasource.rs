/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct; 

use js::rust::HandleObject;

use crate::script_runtime::CanGc;
use crate::dom::bindings::root::DomRoot;
use crate::dom::mediasource::MediaSource;
use crate::dom::window::Window;
use crate::dom::bindings::codegen::Bindings::ManagedMediaSourceBinding::ManagedMediaSourceMethods;

#[dom_struct]
pub(crate) struct ManagedMediaSource {
  media_source: MediaSource,
}

impl ManagedMediaSourceMethods<crate::DomTypeHolder> for ManagedMediaSource {
  fn Constructor(_window: &Window, _proto: Option<HandleObject>, _can_gc: CanGc) -> DomRoot<ManagedMediaSource> {
    todo!()
  }

  fn Streaming(&self) -> bool {
      todo!()
  }

  event_handler!(startstreaming, GetOnstartstreaming, SetOnstartstreaming);
  event_handler!(endstreaming, GetOnendstreaming, SetOnendstreaming);
}