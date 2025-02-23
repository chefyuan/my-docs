C++标准库没有为`std::string`提供`split`方法，至于为什么没有提供，我查了很多资料，网上也有很多说法，但是我依旧没找到比较满意的答案。

肯定不是因为难以实现就是了，大概率是因为加了`split`会打破某种设计的哲学。

官方没有，但是还经常有这个需求（项目中，平时刷题，都会用到`split`），那就只能自己实现一个。

这里贴一个我经常在用的一个`split`的实现：

```C++
std::vector<std::string> stringSplit(const std::string& str, char delim) {
    std::stringstream ss(str);
    std::string item;
    std::vector<std::string> elems;
    while (std::getline(ss, item, delim)) {
        if (!item.empty()) {
            elems.push_back(item);
        }
    }
    return elems;
}
```

还有种常见的方式是使用C语言的`strtok`：

```C++
std::vector<std::string> stringSplit(const std::string& strIn, char delim) {
    char* str = const_cast<char*>(strIn.c_str());
    std::string s;
    s.append(1, delim);
    std::vector<std::string> elems;
    char* splitted = strtok(str, s.c_str());
    while (splitted != NULL) {
        elems.push_back(std::string(splitted));
        splitted = strtok(NULL, s.c_str());
    }
    return elems;
}
```

如果你想使用`split`却不知道怎么实现，可以考虑从以上两种方式中挑一个，我更多的是用第一种。