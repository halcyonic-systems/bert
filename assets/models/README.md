# BERT Example Models

This directory contains example BERT models that are bundled with the application and accessible through the Model Browser.

## Model Format

Models are stored as JSON files following the BERT WorldModel structure:

```json
{
  "systems": [...],
  "interactions": [...],
  "environment": {...},
  "hidden_entities": [],
  "is_same_as_id_counter": 0
}
```

## Planned Models

- **cell.json** - A simple biological cell system
- **organization.json** - An organizational structure example
- **circuit.json** - A basic electrical circuit

## Adding New Models

1. Create a model in BERT
2. Save it as a .json file
3. Place it in this directory
4. Update the Model Browser component to include it

## Notes

Currently waiting for example models to be created using the BERT UI.