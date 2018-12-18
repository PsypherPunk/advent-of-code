#!/bin/bash

PREFIX="..............................."
SUFFIX="..............................."
STATE="${PREFIX}$(head -n1 input.txt | cut -d " " -f3)${SUFFIX}"
NEW_STATE=${STATE}

echo "Start: ${STATE}"

declare -A NOTES

while read input
do
    INPUT=(${input})
    NOTES[${INPUT[0]}]=${INPUT[2]}
done < <(grep "=" input.txt)

for generation in {1..20}
do
    for slice in $(seq 0 $((${#STATE} - 5)))
    do
        POTS=${STATE:${slice}:5}
        if [[ -n ${NOTES[${POTS}]} ]]
        then
            NEW_STATE="${NEW_STATE:0:$((${slice} + 2))}${NOTES[${POTS}]}${NEW_STATE:$((${slice} + 3))}"
        fi
    done
    echo "$(printf %02d ${generation}):    ${NEW_STATE}"
    STATE=${NEW_STATE}
done

SUM=0
for pot in $(seq ${#PREFIX} $((${#STATE})))
do
    [[ ${STATE:${pot}:1} == "#" ]] && SUM=$((${SUM} + ${pot} - ${#PREFIX}))
done

echo "Sum:   ${SUM}"
