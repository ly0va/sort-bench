#include <bits/stdc++.h>
#include <parallel/algorithm>

typedef std::vector<int> vec;
const int LEN = 100000000;

vec merge(const vec &arr1, const vec &arr2) {
    vec result(arr1.size() + arr2.size());
    unsigned i = 0, j = 0;
    while (i < arr1.size() && j < arr2.size()) {
         result[i+j] = arr1[i] < arr2[j] ? arr1[i++] : arr2[j++];
    }
    copy(arr1.begin()+i, arr1.end(), result.begin()+i+j);
    copy(arr2.begin()+j, arr2.end(), result.begin()+i+j);
    return result;
}

vec merge_sort(const vec &arr) {
    if (arr.size() <= 1) return arr;
    int mid = arr.size() / 2;
    vec left_half = merge_sort(vec(arr.begin(), arr.begin()+mid));
    vec right_half = merge_sort(vec(arr.begin()+mid, arr.end()));
    return merge(left_half, right_half);
}

void par_merge_sort(const vec& arr, vec& out) {
    if (arr.size() <= (1 << 13)) {
        out = merge_sort(arr);
    } else {
        int mid = arr.size() / 2;
        vec left_half, right_half;
        std::thread left(
            par_merge_sort, 
            vec(arr.begin(), arr.begin()+mid), 
            std::ref(left_half)
        );
        par_merge_sort(vec(arr.begin()+mid, arr.end()), right_half);
        left.join();
        out = merge(left_half, right_half);
    }
}

double bench(int size, std::function<void(vec&)> sorter) {
    srand(18);
    vec unsorted(size);
    timespec start, finish;
    std::generate(unsorted.begin(), unsorted.end(), rand);
    clock_gettime(CLOCK_MONOTONIC, &start);
    sorter(unsorted);
    clock_gettime(CLOCK_MONOTONIC, &finish);
    double elapsed = (finish.tv_sec - start.tv_sec) +
                     (finish.tv_nsec - start.tv_nsec) / 1e9;
    return elapsed;
}

int main(int argc, char **argv) {
    int len = argc == 1 ? LEN : atoi(argv[1]);
    std::cout << std::fixed << std::setprecision(9);
    double multi = bench(len, [](vec& v) {
        vec out;
        par_merge_sort(v, out);
    });
    double single = bench(len, [](vec& v) {
        merge_sort(v);
    });
    double builtin = bench(len, [](vec& v) {
        std::sort(v.begin(), v.end());
    });
    double builtin_multi = bench(len, [](vec& v) {
        __gnu_parallel::sort(v.begin(), v.end());
    });
    std::cout << "Multi-threaded:   " << multi         << "s\n" 
              << "Single-threaded:  " << single        << "s\n"
              << "Builtin sort:     " << builtin       << "s\n"
              << "Builtin parallel: " << builtin_multi << "s\n";
    return 0;
}

