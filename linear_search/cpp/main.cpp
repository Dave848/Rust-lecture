#include <iostream>
using namespace std;

int linear_search(int arr[], int size, int target) {
    for (int i = 0; i < size; i++) {
        if (arr[i] == target) {
            return i;
        }
    }
    return -1;
}

int main() {
    int arr[] = {10, 20, 30, 40};
    int size = 4;

    int result = linear_search(arr, size, 30);

    if (result != -1)
        cout << "Found at index " << result << endl;
    else
        cout << "Not found" << endl;
}


