# Krunker Client (Tauri)

A minimal Krunker.io client built with Tauri. This client automatically blocks ads and provides a clean gameplay experience without additional features or modifications.

## Features

- Ad blocking is built-in and active by default.
- Lightweight and minimal interface.
- Uses Tauri and WebView2 for rendering.

## Limitations

### Unlimited FPS Unsupported

This client is not suitable for unlimited FPS gameplay.

- WebView2 does not support Chromium version 85.0.4183.121, which is required for stable unlimited FPS in Krunker.
- Microsoft only provides WebView2 runtimes based on newer Chromium versions (e.g., 133+, 134+).
- Attempting to downgrade Chromium manually causes the window to crash. The exact cause of the crash has not been investigated, but it is likely due to incompatibility with outdated Chromium builds.

## Technical Notes

- The client is based on Tauri.
- Ads are blocked using an injected JavaScript script.
- WebView2 runtime is used for rendering (Microsoft Edge-based).

## Summary

This client is functional and blocks all ads by default. However, it **cannot** be used for unlimited FPS due to WebView2 limitations and the lack of support for older Chromium versions.
