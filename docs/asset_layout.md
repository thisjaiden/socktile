# Asset Layout

| Directory / Location | Used By | Details |
| -------------------- | ------- | ------- |
| /core.ldtk | Server  | Default map layout |
| /nothing.png | Client | Used as a placeholder or blank state |
| /audio | Client | All audio assets, in .ogg format |
| /core | Client | Bits and pieces that are *not* modularaly loaded, mostly backgrounds |
| /font | Client | All font assets, in .ttf format (these are non-modular assets) |
| /item | Client | Item icon assets, in .png format (these are non-modular assets) |
| /lang | Client/Server | Text and language assets, in [.ljson](./json.md) format |
| /metadata/audio.sjson | Client | Describes the location and metadata of audio files and assets to be loaded |
| /metadata/terrain[.tjson](./json.md) | Client | Describes a priority order and metadata for different terrain types |
| /metadata/transitions.ujson | Client | Describes the location and metadata of .vjson files, which load terrain spritemaps |
| /npc | Client | Contains images used for NPCs, in .png format (these are non-modular assets) |
| /object | Client | Contains images used for objects, in .png format (these are non-modular assets) |
| /player | Client | Contains images used for the player, in .png format (these are non-modular assets) |
| /terrain | Client | Contains both .pngs and [.vjson](./json.md)s for various terrain state's spritemaps and metadata, respectively |
| /ui | Client | Contains images used for UI elements, in .png format (these are non-modula assets) |
