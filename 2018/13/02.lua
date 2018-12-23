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

function draw()
    local tracks_ = {}
    for x = 1, #tracks do
        tracks_[x] = {}
        for y = 1, #tracks[x] do
            tracks_[x][y] = tracks[x][y]
        end
    end
    for i = 1, #carts do
        if carts[i].crashed then
            tracks_[carts[i].y][carts[i].x] = "X"
        else
            tracks_[carts[i].y][carts[i].x] = "■"
        end
    end
    for i = 1, #tracks_ do
        for j = 1, #tracks_[i] do
            io.write(tracks_[i][j])
        end
        io.write("\n")
    end
end

function sleep(seconds)
    local end_ = os.clock() + seconds
    repeat until os.clock() > end_
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
                    c.crashed = true
                    carts[j].crashed = true
                    break
                end
            end
        end
    end
    local not_crashed = {}
    for n = 1, #carts do
        if not carts[n].crashed then
            not_crashed[#not_crashed + 1] = carts[n]
        end
    end
    if #not_crashed == 1 then
        print("Last crash: ", not_crashed[1].x..","..not_crashed[1].y)
        os.exit()
    end
    draw()
    sleep(0.1)
end

