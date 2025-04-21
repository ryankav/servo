/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// https://w3c.github.io/media-source/#bufferedchangeevent-interface
[Exposed=(Window,DedicatedWorker), Pref="dom_media_source_extension_enabled"]
interface BufferedChangeEvent : Event {
  constructor(DOMString type, optional BufferedChangeEventInit eventInitDict = {});

  [SameObject] readonly attribute TimeRanges addedRanges;
  [SameObject] readonly attribute TimeRanges removedRanges;
};

dictionary BufferedChangeEventInit : EventInit {
  TimeRanges addedRanges;
  TimeRanges removedRanges;
};
