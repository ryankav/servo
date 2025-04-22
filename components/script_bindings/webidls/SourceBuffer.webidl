/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// https://w3c.github.io/media-source/#sourcebuffer
enum AppendMode { "segments", "sequence", };

// FIXME: Currently the tracks extensions do not have DedicatedWorker exposure there is an open
// issue here: https://github.com/w3c/media-source/issues/280 but seems like TimeRanges also are
// lacking the required exposure
[Exposed=(Window/*, DedicatedWorker*/), Pref="dom_media_source_extension_enabled"]
interface SourceBuffer : EventTarget {
  attribute AppendMode mode;
  readonly  attribute boolean updating;
  readonly  attribute TimeRanges buffered;
  attribute double timestampOffset;
  readonly  attribute AudioTrackList audioTracks;
  readonly  attribute VideoTrackList videoTracks;
  readonly  attribute TextTrackList textTracks;
  attribute double appendWindowStart;
  attribute unrestricted double appendWindowEnd;

  attribute EventHandler onupdatestart;
  attribute EventHandler onupdate;
  attribute EventHandler onupdateend;
  attribute EventHandler onerror;
  attribute EventHandler onabort;

  undefined appendBuffer(BufferSource data);
  undefined abort();
  undefined changeType(DOMString type);
  undefined remove(double start, unrestricted double end);
};
