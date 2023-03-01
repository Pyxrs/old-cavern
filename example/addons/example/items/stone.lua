Item.id = 2

Item.model = {
    model = "block",
    block = "stone",
}

function Item:on_use(target, hand)
    Item:place("stone")
end

function Item:on_update(location)
    return
end
