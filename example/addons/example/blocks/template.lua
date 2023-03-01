Block.id = -1

Block.model = {
    model = "cube", -- BTW specify the namespace if you want but it defaults to current namespace
    up = "grass_top",
    down = "dirt",
    side = "grass_side",
}

Block.states = {
    { type = rotation, axis = "x", placement = PlacementState.Random },
}

function Block:on_random_update(visible)
    return
end

function Block:on_neighbor_update(visible)
    return
end

function Block:on_place()
    return
end

function Block:on_destroy()
    return
end

function Block:on_interact()
    return
end

function Block:on_collision()
    return
end

function Block:can_place()
    return true
end

function Block:can_interact()
    return false
end

Block.bounding_box = {
    { 0.0, 0.0, 0.0 },
    { 1.0, 1.0, 1.0 }
}
Block.pathfinding_state = PathfindingState.Solid
