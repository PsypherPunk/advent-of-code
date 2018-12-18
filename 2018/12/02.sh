#!/bin/bash

PREFIX="....."
SUFFIX="....."
STATE="${PREFIX}$(head -n1 input.txt | cut -d " " -f3)${SUFFIX}"
NEW_STATE=${STATE}

echo "Start: ${STATE}"

declare -A NOTES

while read input
do
    INPUT=(${input})
    NOTES[${INPUT[0]}]=${INPUT[2]}
done < <(grep "=" input.txt)

for generation in {1..5000}
do
    for slice in $(seq 0 $((${#STATE} - 5)))
    do
        POTS=${STATE:${slice}:5}
        if [[ -n ${NOTES[${POTS}]} ]]
        then
            NEW_STATE="${NEW_STATE:0:$((${slice} + 2))}${NOTES[${POTS}]}${NEW_STATE:$((${slice} + 3))}"
        fi
    done
    SUM=0
    for pot in $(seq ${#PREFIX} $((${#STATE})))
    do
        [[ ${STATE:${pot}:1} == "#" ]] && SUM=$((${SUM} + ${pot} - ${#PREFIX}))
    done
    echo "$(printf %02d ${generation}):    ${SUM}"
    STATE=${NEW_STATE}
    if [[ ${STATE:0:5} == *#* ]]
    then
        PREFIX=".....${PREFIX}"
        STATE=".....${STATE}"
    fi
    if [[ ${STATE:$((${#STATE} - 5))} == *#* ]]
    then
        SUFFIX="${SUFFIX}....."
        STATE="${STATE}....."
    fi
done

"""
...
753:    20221
754:    20247
755:    20273
756:    20299

$ echo $((50000000000-756))
49999999244
$ echo $((20299 + $(($((50000000000-755)) * 26))))
1300000000669
"""