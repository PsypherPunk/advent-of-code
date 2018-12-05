// Open /2018/day/5/input and run in the console.

let polymer  = $("pre").innerText.trim();

for (let i = 0; i < polymer.length; i++) {
    const ordiff = Math.abs(polymer.charCodeAt(i) - polymer.charCodeAt(i + 1));
    if (ordiff === 32) {
        polymer = `${polymer.slice(0, i)}${polymer.slice(i + 2)}`;
        i -= 2;
    }
}

console.log(polymer.length);

