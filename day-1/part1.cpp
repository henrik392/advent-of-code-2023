#include <fstream>
#include <iostream>

using namespace std;

int main() {
    ifstream input;
    input.open("input.txt");

    int sum = 0;
    string line;
    while (input >> line) {
        int first = -1, second;
        for (char c : line) {
            if (isdigit(c)) {
                if (first == -1)
                    first = c - '0';

                second = c - '0';
            }
        }
        sum += first * 10 + second;
    }

    cout << sum << endl;

    input.close();
    return 0;
}