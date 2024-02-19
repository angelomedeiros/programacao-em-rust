#include <iostream>

using namespace std;

int main(int argc, char const *argv[])
{
    int x = 10;
    int &r = x;
    int s = x;
    int *y = &x;

    r = 20;
    *y = 30;

    cout << "x: " << x << ", "    // 30
         << "r: " << r << ", "    // 30
         << "s: " << s << ", "    // 10
         << "y: " << y << ", "    // 0x16d81320c
         << "*y: " << *y << ", "; // 30
    return 0;
}
