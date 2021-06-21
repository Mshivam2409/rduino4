#include <bits/stdc++.h>
using namespace std;

bool eraseSubStr(std::string &mainStr, const std::string &toErase)
{
    size_t pos = mainStr.find(toErase);
    if (pos != std::string::npos)
    {
        mainStr.replace(pos, toErase.length(), string(toErase.length(), '*'));
        return true;
    }
    return false;
}

vector<int> findSubstring(string A, const vector<string> &B)
{
    unordered_map<string, int> mp;
    vector<int> v;
    int len_req = 0;
    for (int i = 0; i < B.size(); i++)
    {
        len_req += B[i].size();
    }
    for (int i = 0; i <= A.size() - len_req; i++)
    {
        string s = A.substr(i, len_req);
        bool res = true;
        for (int j = 0; j < B.size(); j++)
        {
            string k = B[j];
            eraseSubStr(s, k);
        }
        if (s == string(len_req, '*'))
        {
            v.push_back(i);
        }
    }
    return v;
}

int main()
{

    string A = "abbaccaaabcabbbccbabbccabbacabcacbbaabbbbbaaabaccaacbccabcbababbbabccabacbbcabbaacaccccbaabcabaabaaaabcaabcacabaa";
    vector<string> B = {"cac", "aaa", "aba", "aab", "abc"};
    // string x = "aaab";
    // eraseSubStr(x, "a");
    // cout << x;
    auto p = findSubstring(A, B);
    for (auto x : p)
    {
        cout << x << " ";
    }
}
