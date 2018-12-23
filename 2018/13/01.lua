#!/usr/bin/env lua5.3

local tracks = {}
local carts = {}

local y = 0
for line in io.lines("input.txt") do
    tracks[y] = {}
    local x = 0
    for c in line:gmatch(".") do
        if c == "^" then
            carts[#carts + 1] = {
                x = x,
                y = y,
                dx = 0,
                dy = -1,
                turn = 0
            }
            c = "|"
        elseif c == "v" then
            carts[#carts + 1] = {
                x = x,
                y = y,
                dx = 0,
                dy = 1,
                turn = 0
            }
            c = "|"
        elseif c == ">" then
            carts[#carts + 1] = {
                x = x,
                y = y,
                dx = 1,
                dy = 0,
                turn = 0
            }
            c = "-"
        elseif c == "<" then
            carts[#carts + 1] = {
                x = x,
                y = y,
                dx = -1,
                dy = 0,
                turn = 0
            }
            c = "-"
        end
        tracks[y][x] = c
        x = x + 1
    end
    y = y + 1
end

while true do
    table.sort(carts, function(a, b)
        if a.y == b.y then
            return a.x < b.x
        else
            return a.y < b.y
        end
    end)
    for i = 1, #carts do
        local c = carts[i]
        if not c.crashed then
            c.y = c.y + c.dy
            c.x = c.x + c.dx
            local cell = tracks[c.y][c.x]
            if cell == "+" then
                if c.turn == 0 then
                    c.dx, c.dy = c.dy, -c.dx
                elseif c.turn == 2 then
                    c.dx, c.dy = -c.dy, c.dx
                end
                c.turn = (c.turn + 1) % 3
            elseif cell == "/" then
                if c.dx == 0 then
                    c.dx, c.dy = -c.dy, 0
                else
                    c.dx, c.dy = 0, -c.dx
                end
            elseif cell == "\\" then
                if c.dx == 0 then
                    c.dx, c.dy = c.dy, 0
                else
                    c.dx, c.dy = 0, c.dx
                end
            end
            for j = 1, #carts do
                if i ~= j and carts[j].x == c.x and carts[j].y == c.y and not carts[j].crashed then
                    print("First crash: ", c.x..","..c.y)
                    os.exit()
                end
            end
        end
    end
end

