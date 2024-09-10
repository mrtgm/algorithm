#include <bits/stdc++.h>

using namespace std;

int main()
{
    string X;
    cin >> X;
    vector<int> pos(26);
    for (int i = 0; i < 26; ++i)
    {
        pos[X[i] - 'a'] = i;
    }

    int N;
    cin >> N;
    vector<string> S(N);
    for (string &s : S)
    {
        cin >> s;
    }

    sort(begin(S), end(S), [&](const string &s, const string &t)
         {
        // 文字列の比較
        int len = min(size(s), size(t));
        for (int i = 0; i < len; ++i) {
            if (s[i] != t[i]) {
                return pos[s[i] - 'a'] < pos[t[i] - 'a'];
            }
        }
        return size(s) < size(t); });

    for (const string &s : S)
    {
        cout << s << '\n';
    }
}
