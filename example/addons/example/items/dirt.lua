Item.id = 1

Item.model = {
    model = "block",
    block = "dirt",
}

function Item:on_use(target, hand)
    Item:place("dirt")
end

function Item:on_update(location)
    return
end
