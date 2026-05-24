#include <iostream>
#include <unordered_map>
using namespace std;

unordered_map<int, int> count_frequency(int arr[], int size) {
    unordered_map<int, int> freq;

    for (int i = 0; i < size; i++) {
        freq[arr[i]]++;
    }

    return freq;
}

int main() {
    int data[] = {1, 2, 2, 3, 3, 3, 4};
    int size = 7;

    auto result = count_frequency(data, size);

    for (auto pair : result) {
        cout << pair.first << " -> " << pair.second << endl;
    }
}

