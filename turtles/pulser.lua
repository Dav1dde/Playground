local tArgs = { ... }

function main()
    local side = tArgs[1]
    local sleep = tonumber(tArgs[2])
    local inb = tonumber(tArgs[3])
    inb = inb or 0.1

    if not side and not sleep then
        term.write("pulser side sleep [pause]")
        return
    end

    while true do
        redstone.setOutput(side, true)
        os.sleep(inb)
        redstone.setOutput(side, false)
        os.sleep(sleep)
    end
end

main()