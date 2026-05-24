#include <iostream>
#include <vector>
#include <queue>
#include <limits>
#include <chrono>

using namespace std;
using namespace std::chrono;

typedef pair<int, int> pii;

void dijkstra(int start, const vector<vector<pii>>& graph) {
    int n = graph.size();
    vector<int> dist(n, numeric_limits<int>::max());
    priority_queue<pii, vector<pii>, greater<pii>> pq;

    dist[start] = 0;
    pq.push({0, start});

    while (!pq.empty()) {
        auto [d, u] = pq.top();
        pq.pop();

        if (d > dist[u]) continue;

        for (auto [weight, v] : graph[u]) {
            if (dist[u] + weight < dist[v]) {
                dist[v] = dist[u] + weight;
                pq.push({dist[v], v});
            }
        }
    }
}

int main() {
    int n = 100'000'000;
    vector<vector<pii>> graph(n);

    // Generate synthetic graph
    for (int i = 0; i < n - 1; i++) {
        graph[i].push_back({1, i + 1});
    }

    auto start = high_resolution_clock::now();
    dijkstra(0, graph);
    auto end = high_resolution_clock::now();

    std::cout << "C++ Dijkstra: "
              << duration_cast<milliseconds>(end - start).count()
              << " ms\n";

    return 0;
}