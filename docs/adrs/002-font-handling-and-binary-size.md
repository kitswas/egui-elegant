# ADR 002: Font Handling & Binary Size Optimization

## Status

Accepted

## Context

The crate originally bundled multiple hardcoded `.ttf` font files via `include_bytes!`. This caused the packaged `.crate` size to balloon to over 40MB, violating `crates.io`'s strict 10MB limit and bloating users' binaries unnecessarily. Furthermore, the bundled fonts lacked comprehensive multilingual/Unicode fallback support.

## Decision

1. Removed all bundled heavy font files from the default compilation path.
2. Introduced a lightweight default font option (Monaspace) that is feature-gated under the `monaspace` feature (enabled by default).
3. Added an optional `noto` cargo feature utilizing the `noto-fonts-dl` crate. This allows developers to explicitly opt-in to robust multilingual and emoji text shaping without forcing the download payload on minimal projects.
4. Abstracted font choices behind the `ElegantFont` enum (supporting `System` fonts when all font features are disabled).

## Consequences

- **Pros:** Binary size drastically reduced (the base crate size drops under 100KB when default features are disabled). The crate is fully publishable to `crates.io`. Users have granular control over font payloads.
- **Cons:** Users requiring full internationalization must opt into the `noto` feature. Users wanting absolute minimal binary size must use `default-features = false`.
