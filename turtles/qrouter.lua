
local tArgs = { ... }

function main()
    local inp = tArgs[1]
    local outp = tArgs[2]
    local interval = tonumber(tArgs[3])

    while true do
        if redstone.getInput(inp) then
            if redstone.getInput(inp) then
                redstone.setOutput(outp, true)
                os.sleep(0.1)
                redstone.setOutput(outp, false)
            end

            os.sleep(interval)
        end

        os.sleep(0)
    end

end

main()