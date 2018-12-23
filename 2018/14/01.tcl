#!/usr/bin/tclsh8.6

set fp [open "input.txt" r]
set a_few_recipes [string trim [read $fp]]
close $fp

set board "37"
set step(1) 0
set step(2) 1
set recipes_needed [expr {$a_few_recipes + 10}]

set recipes_made 0
while {$recipes_made < $recipes_needed} {
    set new_recipes 0
    foreach elf {1 2} {
        set score($elf) [string index $board $step($elf)]
        incr new_recipes $score($elf)
    }
    append board $new_recipes

    foreach elf {1 2} {
        set step($elf) [expr {($step($elf) + $score($elf) + 1) % [string length $board]}]
    }

    incr recipes_made
}
puts "[string range $board $a_few_recipes [expr {$a_few_recipes + 9}]]"
