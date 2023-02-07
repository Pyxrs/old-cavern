# Cavern
Voxel engine

## Design Principles

### Storage
Can save and load any file (string or bytes or other) to a specified place in the program's directory.

### Module
A threaded process that is in charge of specific functionality that can provide information for other modules.

Modules can query a module for a crossbeam_channel sender on initialization or runtime in order to send events to that module

Each module should be stored in an Arc to be passed to modules that need it

### Registry
Can be queried for any feature in the game

Modifiable at runtime

Optional loading of features from directory‌

Each feature is stored under a namespace and an ID. If a feature is registered in the same category with the same namespace and ID as another, the newest registry will always be preferred.