function move_n(n)
    for i=0,n do
        move()
    end
end

function move()
    while not turtle.forward() do
    end
end

function up()
    while not turtle.up() do
    end
end

function down()
    while not turtle.down() do
    end
end

function turn_left_right_odd(n)
    if (n % 2) == 1 then
        turtle.turnLeft()
    else
        turtle.turnRight()
    end
end

function plant_tree()
    for i=13,16 do
        turtle.select(i)
        turtle.placeDown()

        if turtle.detectDown() then
            break
        end
    end

    return turtle.detectDown()
end

function suck()
    turtle.select(13)
    turtle.suckDown()
end


function harvest()
    turtle.select(1)
    turtle.dig()
    turtle.forward()
    turtle.digDown()

    local moved_up = 0

    while turtle.detectUp() do
        turtle.digUp()
        if turtle.up() then
            moved_up = moved_up + 1
        end
    end

    if moved_up > 0 then
        for i=0,(moved_up-1) do
            down()
        end

        return plant_tree()
    end

    return false
end

function harvest_row(length)
    local moved = 0

    while length > moved do
        if not turtle.forward() then
            harvest()
        end

        suck()

        moved = moved + 1
    end
end

function reinit()
    for slot=1,12 do
        turtle.select(slot)
        turtle.dropDown()
    end

    move()
    turtle.select(13)

    while turtle.suckDown() do
    end

    up()
end


local tArgs = { ... }

function main()
    local row_length = tonumber(tArgs[1])
    local row_width = tonumber(tArgs[2])
    local rows = tonumber(tArgs[3])

    if not row_length or not row_width or not rows then
        term.write("Call me with a row length, row width and the total number of rows")
        return
    end

    if rows == 1 then
        row_width = 1
    end

    while true do
        reinit()

        for row=0,rows-1 do
            harvest_row(row_length)

            if row < rows-1 then
                turn_left_right_odd(row)
                move_n(row_width)
                turn_left_right_odd(row)
            end
        end

        local mod = 0
        if (rows % 2) == 1 then
            turtle.turnRight()
            move()
            turtle.turnRight()
            move_n(row_length)
        else
            move()
            mod = -row_width
        end

        turtle.turnRight()
        move_n(rows * row_width + mod)
        down()
        turtle.turnRight()
    end
end

main()