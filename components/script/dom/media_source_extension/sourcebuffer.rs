/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct; 

use crate::dom::bindings::codegen::Bindings::SourceBufferBinding::{SourceBufferMethods, AppendMode};
use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::num::Finite;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::codegen::UnionTypes::ArrayBufferViewOrArrayBuffer;
use crate::dom::audiotracklist::AudioTrackList;
use crate::dom::videotracklist::VideoTrackList;
use crate::dom::texttracklist::TextTrackList;
use crate::dom::timeranges::TimeRanges;
use crate::dom::eventtarget::EventTarget;
use crate::dom::bindings::str::DOMString;

#[derive(JSTraceable, MallocSizeOf)]
pub(crate) struct PlaceholderOpaqueSourceBuffer;

#[dom_struct]
pub(crate) struct SourceBuffer {
  eventtarget: EventTarget,
  buffer: DomRefCell<PlaceholderOpaqueSourceBuffer>,
}

impl SourceBufferMethods<crate::DomTypeHolder> for SourceBuffer {
  // https://w3c.github.io/media-source/#dom-sourcebuffer-mode
  fn Mode(&self) -> AppendMode {
    todo!() 
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-mode
  fn SetMode(&self, _value: AppendMode) {
    todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-updating
  fn Updating(&self) -> bool {
    todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-buffered
  fn Buffered(&self, ) -> DomRoot<TimeRanges> {
    todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-timestampoffset
  fn TimestampOffset(&self) -> Finite<f64> {
    todo!() 
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-timestampoffset
  fn SetTimestampOffset(&self, _value: Finite<f64>) {
    todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-audiotracks
  fn AudioTracks(&self) -> DomRoot<AudioTrackList> {
    todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-videotracks
  fn VideoTracks(&self) -> DomRoot<VideoTrackList> {
    todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-texttracks
  fn TextTracks(&self) -> DomRoot<TextTrackList> {
    todo!()
  }
  // https://w3c.github.io/media-source/#dom-sourcebuffer-appendwindowstart
  fn AppendWindowStart(&self) -> Finite<f64> {
    todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-appendwindowstart
  fn SetAppendWindowStart(&self, _value: Finite<f64>) {
    todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-appendwindowend
  fn AppendWindowEnd(&self) -> f64 {
    todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-appendwindowend
  fn SetAppendWindowEnd(&self, _value: f64) {
    todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-onupdatestart
  event_handler!(updatestart, GetOnupdatestart, SetOnupdatestart);

  // https://w3c.github.io/media-source/#dom-sourcebuffer-onupdate
  event_handler!(update, GetOnupdate, SetOnupdate);

  // https://w3c.github.io/media-source/#dom-sourcebuffer-onupdateend
  event_handler!(updateend, GetOnupdateend, SetOnupdateend);

  // https://w3c.github.io/media-source/#dom-sourcebuffer-onerror
  event_handler!(error, GetOnerror, SetOnerror);

  // https://w3c.github.io/media-source/#dom-sourcebuffer-onabort
  event_handler!(abort, GetOnabort, SetOnabort);

  // https://w3c.github.io/media-source/#dom-sourcebuffer-appendbuffer
  fn AppendBuffer(&self, _data: ArrayBufferViewOrArrayBuffer) {
    todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-abort
  fn Abort(&self) {
      todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-changetype
  fn ChangeType(&self, _type_: DOMString) {
      todo!()
  }

  // https://w3c.github.io/media-source/#dom-sourcebuffer-remove
  fn Remove(&self, _start: Finite<f64>, _end: f64) {
      todo!()
  }
}
