#include <iostream>
#include <vector>
#include <chrono>

using namespace std;
using namespace std::chrono;

int main() {
    int n = 1000;

    vector<vector<double>> a(n, vector<double>(n, 1.0));
    vector<vector<double>> b(n, vector<double>(n, 2.0));
    vector<vector<double>> c(n, vector<double>(n, 0.0));

    auto start = high_resolution_clock::now();

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            double sum = 0.0;
            for (int k = 0; k < n; k++) {
                sum += a[i][k] * b[k][j];
            }
            c[i][j] = sum;
        }
    }

    auto end = high_resolution_clock::now();

    cout << "C++ matrix multiply: "
         << duration_cast<milliseconds>(end - start).count()
         << " ms\n";

    // checksum
    cout << "checksum: " << c[0][0] << endl;
}