int main(){
    int x = 1; // あるメモリ領域に1を入れる場所を確保し、その場所をxと名付ける。
    int y = x; // あるメモリにxで名付けられた値を入れる場所を確保し、その場所をyと名付ける。
    struct test_t { // 構造体。苦しんで覚えるCとかに書いていそう。
        int x;
        int y;
    };
    struct test_t t = {1, 2};
    struct test_t s = t;

    return 0;
}