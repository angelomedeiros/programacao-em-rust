#include <string>
#include <iostream>

using namespace std;

int main()
{
    string s = "frayed knot";

    cout << "Size: " << s << endl;
    cout << "Size: " << s.size() << endl;
    cout << "Size: " << s.capacity() << endl;
    cout << "Ref: " << &(&s)[13] << endl;

    s += "is the best!";

    cout << "Size: " << s << endl;
    cout << "Size: " << s.size() << endl;
    cout << "Size: " << s.capacity() << endl;
    cout << "Ref: " << &s << endl;

    return 0;
}