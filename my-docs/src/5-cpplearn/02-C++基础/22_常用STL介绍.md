# vector（动态数组）

`vector`是`STL`中的一个序列容器，可以存储任意类型的动态数组。它能够根据需要自动调整其大小。

**动态**数组就是它最大的特点。

**常见API**：

- **构造与初始化：**

```C++
std::vector<int> vec;          // 创建一个空的vector
std::vector<int> vec(10);      // 创建一个包含10个元素的vector，元素初始值为0
std::vector<int> vec(10, 1);   // 创建一个包含10个元素的vector，元素初始值为1
std::vector<int> vec{1, 2, 3}; // 使用初始化列表
```

- **元素访问：**

```C++
int value = vec[i];           // 通过下标访问元素
int value = vec.at(i);        // 通过at函数访问元素，越界时会抛出异常
```

- **修改元素**：

```C++
vec[i] = new_value;           // 通过下标修改元素
```

- **插入与删除**：

```C++
vec.push_back(value);         // 在末尾插入元素
vec.pop_back();               // 删除末尾元素
vec.insert(vec.begin() + i, value); // 在指定位置插入元素
vec.erase(vec.begin() + i);       // 删除指定位置的元素
```

- **大小与容量**：

```C++
size_t size = vec.size();     // 获取元素个数
size_t capacity = vec.capacity(); // 获取当前容量
vec.resize(new_size);         // 调整容器大小
vec.reserve(new_capacity);    // 预留空间
```

**常见用法**：`vector`适用于需要**动态调整大小**的数组场景，如存储不确定数量的数据。

**示例代码**：

```C++
#include <iostream>
#include <vector>

int main() {
    std::vector<int> vec;
    vec.push_back(1);
    vec.push_back(2);
    vec.push_back(3);

    for (int value : vec) {
        std::cout << value << " ";
    }
    std::cout << std::endl;

    return 0;
}
```

提问：`vector`内部占用的内存是堆内存还是栈内存？

# array（数组）

`array`是C++标准库中的一个固定大小的数组容器。

**常见API：**

- **构造与初始化**：

```C++
std::array<int, 3> arr;         // 创建一个包含3个整数的数组，元素初始值不确定
std::array<int, 3> arr = {1, 2, 3}; // 使用初始化列表
```

- **元素访问**：

```C++
int value = arr[i];            // 通过下标访问元素
```

- **修改元素**：

```C++
arr[i] = new_value;            // 通过下标修改元素
```

- **大小**：

```C++
size_t size = arr.size();      // 获取元素个数
```

**常见用法**：`array`适用于**已知大小且不会改变**的数组场景。

**示例代码**：

```C++
#include <iostream>
#include <array>

int main() {
    std::array<int, 3> arr = {1, 2, 3};

    for (int value : arr) {
        std::cout << value << " ";
    }
    std::cout << std::endl;

    return 0;
}
```

提问：`array`内部占用的内存是堆内存还是栈内存？

# string（字符串）

`string`是C++标准库中的一个字符串类，用于表示和操作字符串，**其实它就是个存储不固定数量字符的容器**。

**常见API**：

- **构造与初始化**：

```C++
std::string str;               // 创建一个空的字符串
std::string str = "hello";     // 使用字符串字面量初始化
std::string str(5, 'a');       // 创建一个包含5个'a'的字符串
```

- **元素访问**：

```C++
char ch = str[i];              // 通过下标访问字符
char ch = str.at(i);           // 通过at函数访问字符，越界时会抛出异常
```

- **修改字符串**：

```C++
str[i] = 'b';                  // 通过下标修改字符
str += " world";               // 追加字符串
str.append("!!!");             // 追加字符串
str.insert(5, "C++");          // 在指定位置插入字符串
str.erase(5, 3);               // 删除指定位置的字符
str.replace(5, 3, "C++");      // 替换指定位置的字符
```

- **大小与容量**：

```C++
size_t size = str.size();      // 获取字符串长度
size_t capacity = str.capacity(); // 获取当前容量
str.resize(new_size);          // 调整字符串大小
```

- **查找与比较**：

```C++
size_t pos = str.find("world");    // 查找子字符串的位置
bool is_equal = (str == "hello");   // 比较字符串是否相等
```

**常见用法**：主要就用来存储字符数组，也没啥特殊用法了。

**示例代码**：

```C++
#include <iostream>
#include
 <string>
int main() {
    std::string str = "hello";

    str += " world";
    std::cout << str << std::endl; // 输出：hello world

    size_t pos = str.find("world");
    std::cout << "Position of 'world': " << pos << std::endl; // 输出：6

    return 0;
}
```

提问：`string`内部占用的内存是堆内存还是栈内存？`string`有个`SSO`概念，你知道吗？

# map（映射）

`map`是一个关联容器，用于存储键值对（key-value pairs），其中每个键都是唯一的，并且会自动按键的升序排序。

**常见API**：

- **构造与初始化**：

```C++
std::map<int, std::string> myMap;       // 创建一个空的map
std::map<int, std::string> myMap = {{1, "one"}, {2, "two"}}; // 使用初始化列表
```

- **插入元素**：

```C++
myMap[key] = value;                     // 插入或更新元素
myMap.insert({key, value});              // 插入元素（若键已存在则不插入）
```

- **查找元素**：

```C++
auto it = myMap.find(key);              // 查找元素，返回迭代器
if (it != myMap.end()) {
    std::cout << "Value: " << it->second << std::endl;
} else {
    std::cout << "Key not found" << std::endl;
}
```

- **删除元素**：

```C++
myMap.erase(key);                       // 删除指定键的元素
myMap.erase(it);                        // 删除指定迭代器的元素
```

- **大小与遍历**：

```C++
size_t size = myMap.size();             // 获取元素个数
for (const auto& pair : myMap) {
    std::cout << "Key: " << pair.first << ", Value: " << pair.second << std::endl;
}
```

**常见用法**：`map`适用于需要按键快速查找值的场景，如字典、配置文件。

**示例代码**：

```C++
#include <iostream>
#include <map>

int main() {
    std::map<int, std::string> myMap = {{1, "one"}, {2, "two"}, {3, "three"}};

    // 插入新元素
    myMap[4] = "four";

    // 查找元素
    auto it = myMap.find(3);
    if (it != myMap.end()) {
        std::cout << "Value for key 3: " << it->second << std::endl; // 输出：Value for key 3: three
    }

    // 遍历map
    for (const auto& pair : myMap) {
        std::cout << "Key: " << pair.first << ", Value: " << pair.second << std::endl;
    }

    return 0;
}
```

# unordered_map（无序map）

`unordered_map`和`map`类似，也用于存储键值对，其中每个键都是唯一的。

与`map`不同的是，`unordered_map`不会按键的顺序进行排序，而是使用哈希表来实现，因此具有更快的查找、插入和删除操作。

**操作与常见API**：

- **构造与初始化**：

```C++
std::unordered_map<int, std::string> myUnorderedMap;       // 创建一个空的unordered_map
std::unordered_map<int, std::string> myUnorderedMap = {{1, "one"}, {2, "two"}}; // 使用初始化列表
```

- **插入元素**：

```C++
myUnorderedMap[key] = value;                               // 插入或更新元素
myUnorderedMap.insert({key, value});                       // 插入元素（若键已存在则不插入）
myUnorderedMap.emplace(key, value);                        // 原地构造元素（若键已存在则不插入）
```

- **查找元素**：

```C++
auto it = myUnorderedMap.find(key);                        // 查找元素，返回迭代器
if (it != myUnorderedMap.end()) {
    std::cout << "Value: " << it->second << std::endl;
} else {
    std::cout << "Key not found" << std::endl;
}
```

- **删除元素**：

```C++
myUnorderedMap.erase(key);                                 // 删除指定键的元素
myUnorderedMap.erase(it);                                  // 删除指定迭代器的元素
```

- **大小与遍历**：

```C++
size_t size = myUnorderedMap.size();                       // 获取元素个数
for (const auto& pair : myUnorderedMap) {
    std::cout << "Key: " << pair.first << ", Value: " << pair.second << std::endl;
}
```

**常见用法**：`unordered_map`适用于**不需要按键排序**且需要快速查找、插入和删除操作的场景。个人编程生涯中，`unordered_map`比`map`更常用。

**示例代码**：

```C++
#include <iostream>
#include <unordered_map>

int main() {
    std::unordered_map<int, std::string> myUnorderedMap = {{1, "one"}, {2, "two"}, {3, "three"}};

    // 插入新元素
    myUnorderedMap[4] = "four";

    // 查找元素
    auto it = myUnorderedMap.find(3);
    if (it != myUnorderedMap.end()) {
        std::cout << "Value for key 3: " << it->second << std::endl; // 输出：Value for key 3: three
    }

    // 遍历unordered_map
    for (const auto& pair : myUnorderedMap) {
        std::cout << "Key: " << pair.first << ", Value: " << pair.second << std::endl;
    }

    return 0;
}
```

# list（链表）

`list`是一个双向链表容器，可以在常数时间内进行任意位置的插入和删除操作。

**操作与常见API**：

- **构造与初始化**：

```C++
std::list<int> myList;         // 创建一个空的list
std::list<int> myList = {1, 2, 3}; // 使用初始化列表
```

- **插入元素**：

```C++
myList.push_back(value);       // 在末尾插入元素
myList.push_front(value);      // 在开头插入元素
myList.insert(it, value);      // 在指定迭代器位置插入元素
```

- **删除元素**：

```C++
myList.pop_back();             // 删除末尾元素
myList.pop_front();            // 删除开头元素
myList.erase(it);              // 删除指定迭代器的元素
```

- **遍历**：

```C++
for (auto it = myList.begin(); it != myList.end(); ++it) {
    std::cout << *it << " ";
}
std::cout << std::endl;

// 使用范围for循环遍历
for (int value : myList) {
    std::cout << value << " ";
}
std::cout << std::endl;
```

- **大小与反转**：

```C++
size_t size = myList.size();   // 获取元素个数
myList.reverse();              // 反转list
```

**常见用法**：`list`适用于需要频繁在任意位置进行插入和删除操作的场景，但不适合随机访问元素。

**示例代码**：

```C++
#include <iostream>
#include <list>

int main() {
    std::list<int> myList = {1, 2, 3, 4, 5};

    // 在开头插入元素
    myList.push_front(0);

    // 遍历list
    for (int value : myList) {
        std::cout << value << " ";
    }
    std::cout << std::endl; // 输出：0 1 2 3 4 5

    // 反转list
    myList.reverse();

    // 遍历反转后的list
    for (int value : myList) {
        std::cout << value << " ";
    }
    std::cout << std::endl; // 输出：5 4 3 2 1 0

    return 0;
}
```

注意，`list`不能使用`std::sort`进行排序，应该使用自己的`sort`方法。

# tuple（元组）

`tuple`是C++标准库中的一个固定大小的异类容器，可以将多个不同类型的值组合在一起。

**操作与常见API**：

- **构造与初始化**：

```C++
std::tuple<int, double, std::string> myTuple(1, 2.5, "three"); // 使用构造函数初始化
std::tuple<int, double, std::string> myTuple = std::make_tuple(1, 2.5, "three"); // 使用make_tuple函数初始化
```

- **访问元素**：

```C++
int intValue = std::get<0>(myTuple);       // 通过索引访问元素
double doubleValue = std::get<1>(myTuple); // 通过索引访问元素
std::string strValue = std::get<2>(myTuple); // 通过索引访问元素
```

- **解构赋值**：

```C++
int a;
double b;
std::string c;
std::tie(a, b, c) = myTuple; // 将tuple中的元素解构并赋值给变量
```

- **比较**：

```C++
if (myTuple == std::make_tuple(1, 2.5, "three")) {
    std::cout << "Tuples are equal" << std::endl;
}
```

**常见用法**：`tuple`适用于需要将多个不同类型的值组合在一起并返回的场景，如函数返回多个不同类型的值，当然，你也可以用`struct`。

**示例代码**：

```C++
#include <iostream>
#include <tuple>
#include <string>

int main() {
    std::tuple<int, double, std::string> myTuple = std::make_tuple(1, 2.5, "three");

    // 访问元素
    int intValue = std::get<0>(myTuple);
    double doubleValue = std::get<1>(myTuple);
    std::string strValue = std::get<2>(myTuple);

    std::cout << "Int: " << intValue << ", Double: " << doubleValue << ", String: " << strValue << std::endl;

    // 解构赋值
    int a;
    double b;
    std::string c;
    std::tie(a, b, c) = myTuple;
    std::cout << "Int: " << a << ", Double: " << b << ", String: " << c << std::endl;
    return 0;
}
```

# `deque`（双端队列）

- `deque`（double-ended queue）也是一种序列容器，支持在容器的前端和后端快速地插入和删除元素，还支持下标访问。
- 它类似于`vector`，但`deque`在头部插入和删除元素时更高效。

**常见操作及API**：

- `push_back(value)`：在容器末尾添加元素。
- `push_front(value)`：在容器开头添加元素。
- `pop_back()`：移除容器末尾的元素。
- `pop_front()`：移除容器开头的元素。
- `front()`：访问容器开头的元素。
- `back()`：访问容器末尾的元素。
- `size()`：返回容器中元素的数量。
- `empty()`：检查容器是否为空。

**示例代码**：

```C++
#include <iostream>
#include <deque>

int main() {
    std::deque<int> dq;
    dq.push_back(1);
    dq.push_front(0);
    dq.push_back(2);

    std::cout << "Deque elements: ";
    for (int n : dq) {
        std::cout << n << " ";
    }
    std::cout << std::endl;

    dq.pop_front();
    std::cout << "After pop_front: ";
    for (int n : dq) {
        std::cout << n << " ";
    }
    std::cout << std::endl;

    return 0;
}
```

# `set`（集合）

`set`是一种基于红黑树实现的有序集合，其中元素是唯一的，且自动按升序排序。

**常见操作及API**：

- `insert(value)`：插入元素。
- `erase(value)`：删除元素。
- `find(value)`：查找元素，返回迭代器。
- `count(value)`：返回元素出现的次数（对于`set`，只能是0或1）。
- `size()`：返回元素数量。
- `empty()`：检查是否为空。
- `begin()`/`end()`：返回迭代器范围。

**常见用法**：需要存储唯一且有序的元素时很使用。

**示例代码**：

```C++
#include <iostream>
#include <set>

int main() {
    std::set<int> s;
    s.insert(1);
    s.insert(2);
    s.insert(2); // 重复元素不会被插入

    std::cout << "Set elements: ";
    for (int n : s) {
        std::cout << n << " ";
    }
    std::cout << std::endl;

    s.erase(1);
    std::cout << "After erase: ";
    for (int n : s) {
        std::cout << n << " ";
    }
    std::cout << std::endl;

    return 0;
}
```

# `unordered_set`（无序集合）

`unordered_set`是一种基于哈希表实现的无序集合，其中元素是唯一的，但不保证元素顺序。

**常见操作及API**：

- 与`set`类似，但操作不保证顺序。
- `insert(value)`
- `erase(value)`
- `find(value)`
- `count(value)`
- `size()`
- `empty()`
- 等

**常见用法**：需要快速查找且不关心元素顺序时使用，也不需要排序。

**示例代码**：

```C++
#include <iostream>
#include <unordered_set>

int main() {
    std::unordered_set<int> us;
    us.insert(1);
    us.insert(2);
    us.insert(2); // 重复元素不会被插入

    std::cout << "Unordered set elements: ";
    for (int n : us) {
        std::cout << n << " ";
    }
    std::cout << std::endl;

    us.erase(1);
    std::cout << "After erase: ";
    for (int n : us) {
        std::cout << n << " ";
    }
    std::cout << std::endl;

    return 0;
}
```

# `queue`（队列）

`queue`是一种先进先出（FIFO）的容器适配器，一般是基于`deque`实现。

**常见操作及API**：

- `push(value)`：在队列末尾添加元素。
- `pop()`：移除队列开头的元素。
- `front()`：访问队列开头的元素。
- `back()`：访问队列末尾的元素。
- `size()`：返回元素数量。
- `empty()`：检查是否为空。

**常见用法**：需要按先进先出顺序处理元素时使用。

**示例代码**：

```C++
#include <iostream>
#include <queue>

int main() {
    std::queue<int> q;
    q.push(1);
    q.push(2);
    q.push(3);

    std::cout << "Queue elements: ";
    while (!q.empty()) {
        std::cout << q.front() << " ";
        q.pop();
    }
    std::cout << std::endl;

    return 0;
}
```

# `stack`（栈）

`stack`是一种后进先出（LIFO）的容器适配器，和`queue`类似，一般也是基于`deque`实现。

**常见操作及API**：

- `push(value)`：在栈顶添加元素。
- `pop()`：移除栈顶的元素。
- `top()`：访问栈顶的元素。
- `size()`：返回元素数量。
- `empty()`：检查是否为空。

**常见用法**：需要按后进先出顺序处理元素时使用。

**示例代码**：

```C++
#include <iostream>
#include <stack>

int main() {
    std::stack<int> st;
    st.push(1);
    st.push(2);
    st.push(3);

    std::cout << "Stack elements: ";
    while (!st.empty()) {
        std::cout << st.top() << " ";
        st.pop();
    }
    std::cout << std::endl;

    return 0;
}
```

# 练习

1. 尝试了解每个组件的原理，并自己实现一套。