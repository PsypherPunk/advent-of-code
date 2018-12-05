// Open /2018/day/5/input and run in the console.

function react(polymer) {
    for (let i = 0; i < polymer.length; i++) {
        const ordiff = Math.abs(polymer.charCodeAt(i) - polymer.charCodeAt(i + 1));
        if (ordiff === 32) {
            polymer = `${polymer.slice(0, i)}${polymer.slice(i + 2)}`;
            i -= 2;
        }
    }
    return polymer;
}

let polymer;
let units = {};

for(let unit = "A".charCodeAt(0); unit <= "Z".charCodeAt(0); unit++) {
    polymer = $("pre").innerText.trim();
    polymer = polymer.replace(new RegExp(String.fromCharCode(unit), "gi"), "");
    polymer = react(polymer);
    units[String.fromCharCode(unit)] = polymer.length;
}

const min = Math.min.apply(null, Object.keys(units).map(function (unit) {
    return units[unit];
}));

console.log(min);

