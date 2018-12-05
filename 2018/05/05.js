// Open /2018/day/5/input and run in the console.

let input  = $("pre").innerText.trim();

for (let i = 0; i < input.length; i++) {
    const ordiff = Math.abs(input.charCodeAt(i) - input.charCodeAt(i + 1));
    if (ordiff === 32) {
        input = `${input.slice(0, i)}${input.slice(i + 2)}`;
        i -= 2;
    }
}

console.log(input.length);

