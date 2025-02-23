迭代器是`STL`的核心组件之一，它提供了一种统一的方式来遍历容器中的元素。

而`STL`算法库则提供了大量高效的通用算法，方便程序员操作容器中的数据。

# 迭代器

C++中，`iterator`（迭代器）是个很重要的理念。

有了它，`STL`容器和算法才能解耦，而它可以理解为两者之间联系的一个桥梁。

`iterator`的语义其实和指针比较相似，可以`++`又可以`--`的。

迭代器类似于指针，它用于遍历容器中的元素。

**它的主要特点就是它提供了一种统一的方式来访问容器中的元素，而不需要关心容器的具体实现。**

先看下它的简单使用：

```C++
#include <iostream>
#include <iterator>
#include <vector>
using namespace std;
int main() {
  vector<int> ar = {1, 2, 3, 4, 5};
  for (vector<int>::iterator ptr = ar.begin(); ptr < ar.end(); ptr++) {
    cout << *ptr << " ";
  }
}
```

根据功能的不同，迭代器可以分为以下几类：

- **输入迭代器（****Input** **Iterator）**：支持读取元素，只能单向遍历。
- **输出迭代器（Output Iterator）**：支持写入元素，只能单向遍历。
- **前向迭代器（Forward Iterator）**：支持读取和写入元素，只能单向遍历。
- **双向迭代器（Bidirectional Iterator）**：支持读取和写入元素，可以双向遍历。
- **随机访问迭代器（Random Access Iterator）**：支持读取和写入元素，可以随机访问。

它们之间的关系如图：

![img](https://lb3fn675fh.feishu.cn/space/api/box/stream/download/asynccode/?code=Y2Q5MTBkNTU2NzNlNDQxYWRkYTQ3OWM4NzVlMWYxNWVfMTVidkJoeHB3bk14UFNjU1pJNGo5TEVJSjVCTWJtVUVfVG9rZW46Wlc4YmJCNGdJb0U2VW54SGxOeWNDbUVxbkRlXzE3NDAyODkzMjk6MTc0MDI5MjkyOV9WNA)

图片截自[https://cplusplus.com/reference/iterator/](https://cplusplus.com/reference/iterator)

通过图片你应该就能明白这几种`iterator`的作用和关系了，如果你想知道自己使用的`iterator`是什么类型，可以访问一下它的`iterator_category`，一般的`iterator`都会指定它的`category`，这里不理解没关系，继续往下看，后面有代码示例。

像常见的`STL`，都会实现一套自己的`iterator`方法，主要就是`begin()`，`end()`这种，比如`vector`，这里贴一下`vector`的源码：

![img](https://lb3fn675fh.feishu.cn/space/api/box/stream/download/asynccode/?code=NzI0YzlhNmJmMWU1MTk5ZTk2ZDZiMDZmYjNmYmEyYTNfQkhXYXdMbTlWdHp6RExjUlBRUVFQc1pjZzNZMWxrZWNfVG9rZW46RzFzTWJlQnk0b2Z6OUt4QXlxOGNMNjVXbkNjXzE3NDAyODkzMjk6MTc0MDI5MjkyOV9WNA)

其它的STL也是类似的。

我们自定义`class`时，为了能够适配`STL`的`algorithm`，也需要实现`iterator`相关的方法。

**那****`iterator`****是什么类型？**

这个我们可以自己定义，需要结合实际情况，一般情况下是个指针，比如我之前的代码中是这样定义的：

![img](https://lb3fn675fh.feishu.cn/space/api/box/stream/download/asynccode/?code=NmU2Nzc4YzIzM2FjYTMyZjIzYmVkMGZjZTlhYWFhMWZfdVd2TWV3UmdiajFrc3lXV05kRTkyZnduejc4VEpyUjJfVG9rZW46TVdNTGJmR3ZUbzRMOFp4Mk5JbmN1ekkxblVoXzE3NDAyODkzMjk6MTc0MDI5MjkyOV9WNA)

在`begin()`时候返回首地址即可，然后它`++`、--其实就是指针的`++`、`--`。

当然`iterator`的标准实现比我这个复杂，一般都会单独写一个`iterator`类，然后自己的`class`再依赖它。

具体可以看看这篇文章[https://anderberg.me/2016/07/04/c-custom-iterators/](https://anderberg.me/2016/07/04/c-custom-iterators)

也可以看看这段自定义`iterator`的代码：https://gist.github.com/jeetsukumaran/307264

注意上面我的代码中既有`iterator`，又有`const_iterator`，它俩有什么区别，估计你能猜得到，但这个其实也需要结合语境，比如`string_view`中，它是个视图的概念，`iterator`和`const_iterator`没啥区别，但是在其它语境下可能就不一样了。

你可以通过上面链接中的代码了解下`iterator`和`const_iterator`的区别，也可以看看这个相关的讨论https://stackoverflow.com/questions/3582608/how-to-correctly-implement-custom-iterators-and-const-iterators 。

我再贴一个比较容易理解的自定义`iterator`的代码：

```C++
#include <cstddef>
#include <iterator>
class Integers {
 public:
 struct Iterator {
  using iterator_category = std::forward_iterator_tag;
  using difference_type = std::ptrdiff_t;
  using value_type = int;
  using pointer = int*;
  using reference = int&;
  Iterator(pointer ptr) : m_ptr(ptr) {}
  reference operator*() const { return *m_ptr; }
  pointer operator->() { return m_ptr; }
  Iterator& operator++() {
   m_ptr++;
   return *this;
  }
  Iterator operator++(int) {
   Iterator tmp = *this;
   ++(*this);
   return tmp;
  }
  friend bool operator==(const Iterator& a, const Iterator& b) { return a.m_ptr == b.m_ptr; };
  friend bool operator!=(const Iterator& a, const Iterator& b) { return a.m_ptr != b.m_ptr; };
  private:
  pointer m_ptr;
 };
 Iterator begin() { return Iterator(&m_data[0]); }
 Iterator end() { return Iterator(&m_data[200]); }
 private:
 int m_data[200];
};
```

以上代码来源于https://www.internalpointers.com/post/writing-custom-iterators-modern-cpp

大家一定要确保自己理解了上面代码。

可能有些人会直接使用`std::iterator`，而不是自己`using`或`typedef`一套自己的`iterator`，但值得注意的是`std::iterator`在`C++17`标准中已经被标记为**`deprecated`****。**

![img](https://lb3fn675fh.feishu.cn/space/api/box/stream/download/asynccode/?code=ZWM2NjRiYjhjMmVhNGVhY2Q1MDQ0NDg4OTMzY2IzYzdfYURudlFEamxkaVowT1hDUXVZQkpXRThsQ0J2UlBDbUhfVG9rZW46S1FnS2JIa0dTbzd3dll4WmZxVGNpRFRubmliXzE3NDAyODkzMjk6MTc0MDI5MjkyOV9WNA)

这是相关的讨论https://stackoverflow.com/questions/37031805/preparation-for-stditerator-being-deprecated/38103394 ，感兴趣的朋友可以看看。

估计在未来`std::iterator`会退出`C++`的舞台，所以还是建议自定义`iterator`。

我们平时编程中可能不一定会用到自定义`iterator`，但是有一点我们需要知道：`iterator`是`container`和`algorithm`之间的桥梁、润滑剂而存在，这个设计理念值得我们学习。

# 迭代器的使用与遍历

## 使用

- **解引用**：使用`*`操作符访问迭代器指向的元素。
- **递增**：使用`++`操作符将迭代器移动到下一个元素。
- **递减**：使用`--`操作符将迭代器移动到上一个元素（仅适用于双向迭代器和随机访问迭代器）。
- **比较**：使用`==`和`!=`操作符比较两个迭代器是否指向同一个元素。

## 遍历

使用迭代器可以方便地遍历容器中的元素。

### 遍历`std::vector`

```C++
#include <iostream>
#include <vector>

int main() {
    std::vector<int> vec = {1, 2, 3, 4, 5};

    // 使用迭代器遍历vector
    for (std::vector<int>::iterator it = vec.begin(); it != vec.end(); ++it) {
        std::cout << *it << " "; // 输出：1 2 3 4 5
    }
    std::cout << std::endl;

    return 0;
}
```

### 遍历`std::list`

```C++
#include <iostream>
#include <list>

int main() {
    std::list<int> lst = {1, 2, 3, 4, 5};

    // 使用迭代器遍历list
    for (std::list<int>::iterator it = lst.begin(); it != lst.end(); ++it) {
        std::cout << *it << " "; // 输出：1 2 3 4 5
    }
    std::cout << std::endl;

    return 0;
}
```

### 遍历`std::map`

```C++
#include <iostream>
#include <map>

int main() {
    std::map<std::string, int> m = {{"Alice", 25}, {"Bob", 30}, {"Charlie", 35}};

    // 使用迭代器遍历map
    for (std::map<std::string, int>::iterator it = m.begin(); it != m.end(); ++it) {
        std::cout << it->first << ": " << it->second << std::endl;
    }
    // 输出：
    // Alice: 25
    // Bob: 30
    // Charlie: 35

    return 0;
}
```

# STL算法库

STL算法库提供了大量高效的通用算法，用于操作容器中的数据。这些算法通常通过迭代器来访问容器中的元素。

迭代器是算法与容器的桥梁。

## `std::for_each`

`std::for_each`用于对容器中的每个元素执行指定的操作。

```C++
#include <iostream>
#include <vector>
#include <algorithm>

void print(int num) {
    std::cout << num << " ";
}

int main() {
    std::vector<int> vec = {1, 2, 3, 4, 5};

    // 使用std::for_each遍历vector
    std::for_each(vec.begin(), vec.end(), print); // 输出：1 2 3 4 5
    std::cout << std::endl;

    return 0;
}
```

## `std::find`

`std::find`用于在容器中查找指定值的元素。

```C++
#include <iostream>
#include <vector>
#include <algorithm>

int main() {
    std::vector<int> vec = {1, 2, 3, 4, 5};

    // 使用std::find查找元素
    std::vector<int>::iterator it = std::find(vec.begin(), vec.end(), 3);
    if (it != vec.end()) {
        std::cout << "Found: " << *it << std::endl; // 输出：Found: 3
    } else {
        std::cout << "Not found." << std::endl;
    }

    return 0;
}
```

## `std::sort`

`std::sort`用于对容器中的元素进行排序。

```C++
#include <iostream>
#include <vector>
#include <algorithm>

int main() {
    std::vector<int> vec = {5, 3, 1, 4, 2};

    // 使用std::sort排序
    std::sort(vec.begin(), vec.end());

    // 输出排序后的vector
    for (int num : vec) {
        std::cout << num << " "; // 输出：1 2 3 4 5
    }
    std::cout << std::endl;

    return 0;
}
```

## `std::copy`

`std::copy`用于将一个容器中的元素复制到另一个容器中。

```C++
#include <iostream>
#include <vector>
#include <algorithm>

int main() {
    std::vector<int> vec1 = {1, 2, 3, 4, 5};
    std::vector<int> vec2(5);

    // 使用std::copy复制元素
    std::copy(vec1.begin(), vec1.end(), vec2.begin());

    // 输出复制后的vector
    for (int num : vec2) {
        std::cout << num << " "; // 输出：1 2 3 4 5
    }
    std::cout << std::endl;

    return 0;
}
```

## `std::transform`

`std::transform`用于对容器中的元素进行转换。

```C++
#include <iostream>
#include <vector>
#include <algorithm>

int square(int num) {
    return num * num;
}

int main() {
    std::vector<int> vec1 = {1, 2, 3, 4, 5};
    std::vector<int> vec2(5);

    // 使用std::transform对元素进行平方运算
    std::transform(vec1.begin(), vec1.end(), vec2.begin(), square);

    // 输出转换后的vector
    for (int num : vec2) {
        std::cout << num << " "; // 输出：1 4 9 16 25
    }
    std::cout << std::endl;

    return 0;
}
```

## `std::accumulate`

`std::accumulate`用于计算容器中元素的累加和。

```C++
#include <iostream>
#include <vector>
#include <numeric>

int main() {
    std::vector<int> vec = {1, 2, 3, 4, 5};

    // 使用std::accumulate计算累加和
    int sum = std::accumulate(vec.begin(), vec.end(), 0);
    std::cout << "Sum: " << sum << std::endl; // 输出：Sum: 15

    return 0;
}
```

# 练习

1. 使用`std::for_each`和`Lambda`表达式遍历一个`std::vector`，并输出每个元素的平方。
2. 使用`std::find`在一个`std::list`中查找指定元素，并输出其位置。
3. 使用`std::sort`对一个`std::deque`进行排序，并输出排序后的结果。
4. 使用`std::copy`将一个`std::vector`中的元素复制到一个`std::list`中。
5. 使用`std::transform`将一个`std::vector`中的字符串转换为大写，并输出结果。