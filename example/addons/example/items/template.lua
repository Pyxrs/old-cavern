Item.id = -1

Item.model = {
    model = "block",
    block = "grass",
}

function Item:on_use(target, hand)
    return
end

function Item:on_update(location)
    return
end

Item.description = {
    "Wow what a great example",
    "Cool"
}
