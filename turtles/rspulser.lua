local tArgs = { ... }

function main()
    local side = tArgs[1]
    local trigger = tArgs[2]
    local pulses = tonumber(tArgs[3])
    pulses = pulses or 1
    local sleep = tonumber(tArgs[4])
    sleep = sleep or 0.1
    local inb = tonumber(tArgs[5])
    inb = inb or 0.1

    if not side and not trigger then
        term.write("rspulser side trigger [pulses, [sleep, [pause]]]")
        return
    end

    while true do
        if redstone.getInput(trigger) then
            for i=1,pulses do
                redstone.setOutput(side, true)
                os.sleep(inb)
                redstone.setOutput(side, false)
                os.sleep(sleep)
            end
        end
        os.sleep(0.1)
    end
end

main()