import * as deno from 'deno'

function game(players: number, max: number) {
    let circle = []
    let elves = new Array(players).fill(0);
    let marbles = Array.from(Array(max + 1).keys())

    let position = 0
    let marble = marbles.shift()
    circle.splice(position, 0, marble)

    while (marbles.length > 0) {
        for (let elf = 0; elf < players; elf++) {
            if (marbles.length === 0) {
                break
            }
            let marble = marbles.shift()
            if (marble % 23 == 0) {
                elves[elf] += marble;
                position -= 7
                elves[elf] += circle.splice(position, 1)[0]
            } else {
                position += 2
                if (position > circle.length) {
                    position = Math.min(1, circle.length)
                }
                circle.splice(position, 0, marble)
            }
            // console.log(`[${elf + 1}]  ${circle.join(' ')}`)
        }
    }

    return elves.reduce((acc, x) => (x > acc ? x : acc), 0);
}

const input = '464 players; last marble is worth 709180 points'
const values = new RegExp('^([0-9]+) players; last marble is worth ([0-9]+) points').exec(input)

console.log(game(parseInt(values[1]), parseInt(values[2])))
