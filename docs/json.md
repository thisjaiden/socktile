# Alternate JSON Files

## Why are there JSON files with weird file extensions?

Bevy decides on the type of an asset partially based on its file extension. Unfortunately this doesn't support extensions like `*.audio.json` so instead each config type using JSON files has a letter prefixed to it.

## When will this be fixed?

See [bevy#367](https://github.com/bevyengine/bevy/issues/367).  
Maybe soon, maybe in a long time, maybe never.

## What are the extensions used?

| Config Type                 | Extension |
| --------------------------- | --------- |
| Language Declarations       | .bjson    |
| Language Data               | .ljson    |
| Audio                       | .sjson    |
| Terrain Variations          | .vjson    |
| Terrain Variation Locations | .ujson    |
| Terrain Metadata            | .tjson    |
| Animated Sprite             | .ajson    |
