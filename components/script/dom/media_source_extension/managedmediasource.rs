/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;

use crate::dom::bindings::codegen::Bindings::ManagedMediaSourceBinding::ManagedMediaSourceMethods;
use crate::dom::bindings::root::DomRoot;
use crate::dom::mediasource::MediaSource;
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct ManagedMediaSource {
    media_source: MediaSource,
}

impl ManagedMediaSourceMethods<crate::DomTypeHolder> for ManagedMediaSource {
    // https://w3c.github.io/media-source/#dom-managedmediasource-constructor
    fn Constructor(
        _window: &Window,
        _proto: Option<HandleObject>,
        _can_gc: CanGc,
    ) -> DomRoot<ManagedMediaSource> {
        todo!()
    }

    // https://w3c.github.io/media-source/#dom-managedmediasource-streaming
    fn Streaming(&self) -> bool {
        todo!()
    }

    // https://w3c.github.io/media-source/#dfn-startstreaming
    event_handler!(startstreaming, GetOnstartstreaming, SetOnstartstreaming);

    // https://w3c.github.io/media-source/#dfn-endstreaming
    event_handler!(endstreaming, GetOnendstreaming, SetOnendstreaming);
}
