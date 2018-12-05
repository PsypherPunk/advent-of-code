#include <fstream>
#include <iostream>
#include <numeric>
#include <regex>

using namespace std;

const char *pattern =
        "^\\[1518-([0-9]{2})-([0-9]{2}) ([0-9]{2}):([0-9]{2})\\] "
        "(falls asleep|wakes up|Guard #([0-9]+) begins shift)$";

bool compare(const pair<int, array<int, 60>>& a, const pair<int, array<int, 60>>& b) {
    return (
            accumulate(a.second.begin(), a.second.end(), 0, plus<>()) <
            accumulate(b.second.begin(), b.second.end(), 0, plus<>())
    );
}

int main() {
    ifstream infile("input.txt");
    string line;
    smatch matches;
    map<int, array<int, 60>> guards;
    int month, day, hour, minute, guard, asleep, wakes;

    regex input(pattern);

    array<string, 986> lines;
    int i;
    while (std::getline(infile, line)) {
        lines[i] = line;
        i++;
    }
    sort(lines.begin(), lines.end());

    for (string line : lines) {
        regex_search(line, matches, input);

        if (matches[6].matched) {
            guard = stoi(matches[6].str());
        }
        if (matches[5].str() == "falls asleep") {
            asleep = stoi(matches[4].str());
        }
        if (matches[5].str() == "wakes up") {
            wakes = stoi(matches[4].str());
            for (int i = asleep; i < wakes; ++i) {
                guards[guard][i]++;
            }
        }
    }

    long sleepy_guard, count_max, minute_max = 0L;
    for (auto guard : guards) {
        auto count = max_element(begin(guards[guard.first]), end(guards[guard.first]));
        if (*count > count_max) {
            count_max = *count;
            minute_max = count - guards[guard.first].data();
            sleepy_guard = guard.first;
        }
    }

    cout << "Guard is most frequently asleep is " << sleepy_guard << endl;
    cout << "He is mostly asleep at minute " << minute_max << endl;
    cout << "So the answer is " << sleepy_guard * minute_max << endl;
}