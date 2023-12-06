#include <fstream>
#include <iostream>
#include <map>

using namespace std;

int main() {
    ifstream input;
    input.open("input.txt");

    const int MAX_DIGIT_SIZE = 5;
    map<string, int> digitDict = {
        {"one", 1},
        {"two", 2},
        {"three", 3},
        {"four", 4},
        {"five", 5},
        {"six", 6},
        {"seven", 7},
        {"eight", 8},
        {"nine", 9}};

    int sum = 0;
    string line;
    cout << "Reading input..." << endl;
    while (input >> line) {
        int first = -1, second = -1;
        for (size_t i = 0; i < line.length(); i++) {
            if (isdigit(line[i])) {
                if (first == -1)
                    first = line[i] - '0';
               second = line[i] - '0';
            } else {
                string digit = "";
                for (size_t j = i; j < line.length() && digit.size() < MAX_DIGIT_SIZE && !digitDict[digit]; j++) {
                    digit += line[j];
                }
                if (digitDict[digit]) {
                    if (first == -1)
                        first = digitDict[digit];
                    second = digitDict[digit];
                }
            }
        }
        // for (size_t i = line.size() - 1; i >= 0; i--) {
        //     if (isalpha(line[i])) {
        //         string digit = "";
        //         for (size_t j = i; i < line.length() && isalpha(line[j]) && digit.size() < MAX_DIGIT_SIZE && !digitDict[digit]; j++) {
        //             digit += line[i];
        //         }
        //         if (digitDict[digit]) {
        //             second = digitDict[digit];
        //             break;
        //         }
        //     } else if (isdigit(line[i])) {
        //         second = line[i] - '0';
        //         break;
        //     }
        // }

        sum += first * 10 + second;
    }

    cout << sum << endl;

    input.close();
    return 0;
}