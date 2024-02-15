#include <vector>
#include <iostream>

using namespace std;

int main()
{
    vector<string> s = {"joao", "maria", "jose"};
    vector<string> t = s; // atribui um novo valor e não uma referencia
    vector<string> u = s; // atribui um novo valor e não uma referencia

    cout << "Ref de s: " << &s << endl; // 0x16fb22480
    cout << "Ref de t: " << &t << endl; // 0x16fb22440
    cout << "Ref de u: " << &u << endl; // 0x16fb22428

    return 0;
}