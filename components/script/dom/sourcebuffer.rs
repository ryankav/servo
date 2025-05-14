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
  fn Updating(&self) -> bool {
    todo!()
  }

  fn Buffered(&self, ) -> DomRoot<TimeRanges> {
    todo!()
  }

  fn Mode(&self) -> AppendMode {
    todo!() 
  }

  fn SetMode(&self, _value: AppendMode) {
    todo!()
  }

  fn TimestampOffset(&self) -> Finite<f64> {
    todo!() 
  }

  fn SetTimestampOffset(&self, _value: Finite<f64>) {
    todo!()
  }

  fn AudioTracks(&self) -> DomRoot<AudioTrackList> {
    todo!()
  }

  fn VideoTracks(&self) -> DomRoot<VideoTrackList> {
    todo!()
  }

  fn TextTracks(&self) -> DomRoot<TextTrackList> {
    todo!()
  }

  fn AppendBuffer(&self, _data: ArrayBufferViewOrArrayBuffer) {
    todo!()
  }

  fn AppendWindowStart(&self) -> Finite<f64> {
    todo!()
  }

  fn SetAppendWindowStart(&self, _value: Finite<f64>) {
    todo!()
  }

  fn AppendWindowEnd(&self) -> f64 {
    todo!()
  }

  fn SetAppendWindowEnd(&self, _value: f64) {
    todo!()
  }

  fn Abort(&self) {
      todo!()
  }

  fn ChangeType(&self, _type_: DOMString) {
      todo!()
  }

  fn Remove(&self, _start: Finite<f64>, _end: f64) {
      todo!()
  }

  event_handler!(updatestart, GetOnupdatestart, SetOnupdatestart);
  event_handler!(update, GetOnupdate, SetOnupdate);
  event_handler!(updateend, GetOnupdateend, SetOnupdateend);
  event_handler!(error, GetOnerror, SetOnerror);
  event_handler!(abort, GetOnabort, SetOnabort);
}