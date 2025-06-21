# BERT Assets

This directory contains static assets used in the BERT application.

## Directory Structure

- `app_icon.ico` - Application icon
- `create-button/` - Icons and images for the creation buttons
- `fonts/` - Typography resources
- `label-icons/` - Icons used for labeling elements

## Asset Guidelines

When adding or modifying assets:

1. **Use appropriate formats**:
   - Icons: SVG preferred, PNG as fallback
   - Fonts: TTF/OTF formats
   - Images: Optimized PNG or JPG files

2. **Maintain organization**:
   - Place assets in the appropriate subdirectory
   - Create new subdirectories for new asset types
   - Use clear, descriptive filenames

3. **Document licensing**:
   - Include license information for third-party assets
   - Respect copyright and attribution requirements

## Asset Usage

Assets are referenced in the codebase via:

- Direct imports in Rust code
- Path references in HTML/CSS
- Relative paths in build configuration

The assets directory is automatically included in the build process through the Trunk.toml configuration.