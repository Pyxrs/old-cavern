Item.id = 0

Item.model = {
    model = "block",
    block = "grass",
}

function Item:on_use(target, hand)
    Item:place("grass")
end

function Item:on_update(location)
    return
end
