Block.namespace = "example"
Block.id = 1

Block.textures = {
    all = "dirt",
}

Block.states = {
    { type = rotation, axis = "x", placement = PlacementState.Random },
    { type = rotation, axis = "y", placement = PlacementState.Random },
    { type = rotation, axis = "z", placement = PlacementState.Random },
}
