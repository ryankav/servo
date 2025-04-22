/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// https://w3c.github.io/media-source/#managedsourcebuffer-interface

// FIXME: exposure from SourceBuffer limits the exposure at present
[Exposed=(Window/*, DedicatedWorker*/), Pref="dom_media_source_extension_enabled"]
interface ManagedSourceBuffer : SourceBuffer {
  attribute EventHandler onbufferedchange;
};
