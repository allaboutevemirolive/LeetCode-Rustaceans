// https://leetcode.com/problems/stream-of-characters/solutions/1216049/c-simple-trie-sol/
bool query(char letter) {
    t=letter+t;
    if(t.size()>2000)  t.resize(2000);
    return query1();
}