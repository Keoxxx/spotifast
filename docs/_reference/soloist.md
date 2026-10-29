---
title: Spotify Soloist
description: Set up Spotify's optional player, with managed downloads and renewal.
nav_order: 5
permalink: /soloist/
---

Available in source builds from `main`; not included in v0.14.2.

Spotifast can run [Spotify Soloist](https://developer.spotify.com/documentation/soloist)
as an optional player. The built-in librespot player remains the default.
Soloist is Spotify's separate software, downloaded from Spotify when you
choose to enable it. It supports lossless playback; its current API does
not report the audio format actually delivered, so Spotifast does not show
a lossless indicator or offer an unsupported quality selector.

## Setup

1. Sign in to Spotify in Spotifast. Spotify Premium is required.
2. Open **Settings → Spotify Soloist**, then **Get your Soloist key from
   Spotify**. Follow Spotify's instructions to generate a personal Soloist
   key. This is separate from the Client ID of your Web API app.
3. Paste the key and choose **Download and enable Soloist**. Spotifast saves
   the key in the system credential store and downloads Spotify's build for
   this computer. No administrator account is needed.
4. Open the official Spotify app on the same network, signed into the same
   account. Choose the exact **Spotifast Soloist (…)** name shown in Settings
   from Spotify's device picker. Spotifast checks that this player appears
   in your signed-in account's device list before controlling it.

Pairing is normally needed only once. Soloist restores its own saved session
when Spotifast starts. You can still choose another Spotify Connect device.
Choose **Turn off Soloist** to stop it. The built-in player remains available
in the device picker.
The saved key and pairing are retained for **Enable with saved key** next time; signing out of Spotifast
removes them. Closing Spotifast completely stops the child player. Closing to
the tray leaves both running.

Spotify publishes Linux x86_64, ARM64 and ARM32 builds, Apple Silicon builds
for macOS 13 or newer, and Windows x86_64 builds. Intel Macs and native
Windows ARM builds are not currently available. Flatpak sandbox restrictions
may prevent the downloaded program from running or reaching the audio service.

## Updates

Soloist builds expire **90 days after their build date**, rather than after
installation. Spotifast reads the build date from the executable. During its
last 30 days it checks daily, while running, for a fresh download. It stages
the replacement without interrupting music, then applies it when the player
is inactive or Spotifast restarts. A paused active session also waits, to
preserve its queue. If Spotify has not published a usable replacement, the
error is shown in Settings; the built-in player remains available.

Downloads come directly from `soloist-builds.spotifycdn.com` over HTTPS.
Spotifast does not bundle or mirror Spotify's executable. Archive extraction
is bounded and rejects links and unsafe paths. A local SHA-256 digest detects
changes to an installed build; it is not a Spotify signature. Spotify does
not currently publish a checksum manifest for independent verification.

## Playback and current limits

Library browsing, search and playlist management keep using Spotifast's
existing Spotify connections. Soloist's loopback API handles basic playback
controls, state and queue reads. Starting a collection at a chosen row goes
through the existing Spotify playback request, retaining its context and
position; it does not skip through earlier songs or shorten the playlist.

Soloist exposes queue additions and reads, but no public command to remove,
reorder, clear or insert at the front of its queue. Those editing controls
remain available with the built-in player. Spotifast does not pretend to
edit a Soloist queue by replacing the playing playlist. Spotify's Connect
queue-write mechanisms may offer a future route, but have not been verified
for Soloist's queue identities and concurrent updates.

Soloist sends audio directly to the operating system's default output. Its
API provides no PCM stream. Spotifast's equalizer, spectrum, oscilloscope,
MilkDrop and local file playback therefore remain features of the built-in
player. Ordinary system audio routing is a possible separate integration:
[PipeWire loopbacks](https://docs.pipewire.org/page_module_loopback.html) can
route a dedicated player output to a processing input on Linux. Windows
[WASAPI loopback](https://learn.microsoft.com/en-us/windows/win32/coreaudio/loopback-recording)
and macOS [Core Audio taps](https://developer.apple.com/documentation/coreaudio/capturing-system-audio-with-core-audio-taps)
need their own capture and output implementations. This is
not implemented, and no claim is made that loopback capture is bit-perfect.
A system-wide loopback would also include unrelated apps, which is unsuitable
for Spotifast's visualizers.

Spotify lists capabilities such as Smart Shuffle, Jam and crossfade for its
player. Spotifast only offers controls exposed by the documented Soloist API;
those names alone do not establish an integration.

## Credentials, privacy and terms

The key belongs to your Spotify account and is stored with Spotifast's other
protected credentials. Soloist currently accepts it only as a command-line
argument, so software permitted to inspect your processes can read it while
Soloist runs. Spotifast never logs the key, command line or child output.
Soloist's pairing state and cache live under the private `soloist/accounts/`
folder in Spotifast's state directory. Its binaries and notices live under
`soloist/builds/`; the `soloist_enabled` setting contains no key.

The local control socket binds only to `127.0.0.1`, on an operating-system
assigned port. Spotify's socket has no client authentication or browser
origin protection. Do not forward its port or expose it to a network.

Soloist makes its own Spotify connections and follows Spotify's privacy
policy. Spotifast's proxy setting applies to the download and Spotifast's
requests, but does not configure Soloist's streaming connections. Spotify's
binary can have different diagnostics and reporting behaviour from librespot.
Spotifast adds no telemetry of its own.

Soloist remains subject to [Spotify's terms](https://developer.spotify.com/terms)
and [Developer Policy](https://developer.spotify.com/policy). Availability of
this optional integration is not a claim of Spotify endorsement or permission
to redistribute its binaries. Spotifast downloads them for the listener from
Spotify's server instead of redistributing them. This integration does not
bypass Spotify's authentication, subscription checks, expiry or DRM.
