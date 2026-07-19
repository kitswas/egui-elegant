# ADR 002: Font Handling & Binary Size Optimization

## Status

Accepted

## Context

The crate originally bundled multiple hardcoded `.ttf` font files via `include_bytes!`. This caused the packaged `.crate` size to balloon to over 40MB, violating `crates.io`'s strict 10MB limit and bloating users' binaries unnecessarily. Furthermore, the bundled fonts lacked comprehensive multilingual/Unicode fallback support.

## Decision

1. Removed all bundled heavy font files from the repository.
2. Introduced a lightweight default font option (Monaspace) that is fetched or bundled minimally.
3. Added an optional `noto` cargo feature utilizing the `noto-fonts-dl` crate. This allows developers to explicitly opt-in to robust multilingual and emoji text shaping without forcing the download payload on minimal projects.
4. Abstracted font choices behind the `ElegantFont` enum.

## Consequences

- **Pros:** Binary size drastically reduced (from 40MB+ to under 100KB). The crate is fully publishable to `crates.io`. Users have granular control over font payloads.
- **Cons:** Users requiring full internationalization must opt into the `noto` feature and accept the runtime/compile-time overhead of downloading fonts.
