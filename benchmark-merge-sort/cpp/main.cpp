#include <iostream>
#include <vector>
#include <thread>
#include <algorithm>
#include <random>
#include <chrono>

using namespace std;
using namespace std::chrono;

void merge(std::vector<int>& arr, int left, int mid, int right) {
    std::vector<int> temp(right - left + 1);

    int i = left, j = mid + 1, k = 0;

    while (i <= mid && j <= right) {
        if (arr[i] <= arr[j]) temp[k++] = arr[i++];
        else temp[k++] = arr[j++];
    }

    while (i <= mid) temp[k++] = arr[i++];
    while (j <= right) temp[k++] = arr[j++];

    std::copy(temp.begin(), temp.end(), arr.begin() + left);
}

void mergeSort(std::vector<int>& arr, int left, int right, int depth = 0) {
    if (left >= right) return;

    int mid = (left + right) / 2;

    if (depth < 4) {
        std::thread t1(mergeSort, std::ref(arr), left, mid, depth + 1);
        std::thread t2(mergeSort, std::ref(arr), mid + 1, right, depth + 1);
        t1.join();
        t2.join();
    } else {
        mergeSort(arr, left, mid, depth + 1);
        mergeSort(arr, mid + 1, right, depth + 1);
    }

    merge(arr, left, mid, right);
}

int main() {    
    const int N = 10'000'000;

    std::vector<int> data(N);
    std::mt19937 rng(42);
    std::uniform_int_distribution<int> dist(0, N);

    for (auto &x : data) x = dist(rng);

    auto start = high_resolution_clock::now();

    mergeSort(data, 0, data.size() - 1);

    auto end = high_resolution_clock::now();

    std::cout << "C++ merge sort: "
              << duration_cast<milliseconds>(end - start).count()
              << " ms\n";

    return 0;
}