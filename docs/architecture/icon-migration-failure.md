# Icon Migration Failure - 2025-11-03

## Issue

Replaced palette icons with properly converted SVGs from `bert-icon-system-collection`, causing BERT to freeze on load with texture format error.

## Error

```
ERROR Device::create_texture error: Texture format Rgba16Unorm can't be used due to missing features
ERROR Handling wgpu errors as fatal by default
```

## Symptoms

- No icons render
- BERT frozen on load
- Main system visible but unresponsive
- Zoom keyboard events detected but not processed

## What Changed

**Commit**: `c8e4a61` - "feat: replace palette icons with proper bert-icon-system-collection assets"

**Converted Icons** (ImageMagick, 40x40, `-background none`):
- subsystem.svg → subsystem.png
- interaction.svg → flow.png, inflow.png, outflow.png
- import.svg → import.png
- export.svg → export.png
- source.svg → source.png
- sink.svg → sink.png

**Icon Mappings Updated** in `palette.rs`:
- ImportInterface: interface.png → import.png
- ExportInterface: interface.png → export.png

## Root Cause Hypothesis

1. **PNG format mismatch**: ImageMagick conversion may have created incompatible PNG format (16-bit vs 8-bit)
2. **Transparency handling**: `-background none` may not produce Bevy-compatible alpha channel
3. **SVG complexity**: Converted SVGs may have features that don't translate well to WebGL context

## Working State

Previous icons (from `assets/create-button/`) worked correctly:
- Simple PNG format
- Already tested in button system
- Known compatible with Bevy 0.15 + WebGL

## Next Steps

1. **Immediate**: Revert to working icons (`git revert c8e4a61`)
2. **Investigation**: Check PNG format of working vs broken icons
3. **Alternative approach**:
   - Copy create-button PNGs directly (proven to work)
   - Or convert SVGs with exact format match to working icons
   - Or research Bevy 0.15 texture format requirements for WASM

## Lessons

- Test icon changes incrementally (one icon at a time)
- Verify PNG format compatibility before bulk replacement
- WASM/WebGL has stricter texture format requirements than native
- Working > Semantically Perfect for Phase 1 validation

## Related

- Phase 1 lessons learned: `docs/architecture/phase1-lessons-learned.md`
- Icon collection: `/Users/home/Desktop/bert/private-dev/bert-icon-system-collection/`
