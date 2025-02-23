在`C/C++`里，**assert**是一种比较常用的调试策略。

# 定义

`assert`字面意思是断言，用于判断是否满足某个条件，如果不满足某个条件，则程序会直接崩溃，控制台上会显示崩溃所在的文件名和行号。

看下它的使用：

```C++
#include <assert.h>
#include <stdio.h>

void func(int a) {
    assert(a > 0);
    printf("%d", a);
}

int main() { func(-1); }
```

示例中我用`assert`来表示函数`func`的输入**一定要>0**。

我如果输入了`-1`，程序运行时会在`assert`处`crash`，同时提供了报错的详细：

```C++
a.out: test.cc:5: void func(int): Assertion `a > 0' failed.
Aborted
```

**注意事项：**

我们一定在`assert`处只做判断，不要添加任何功能逻辑，**因为assert只在debug模式下有作用**，`release`模式下，`assert`这行代码会自动跳过去，会自动被忽略。

```C++
assert(++i > 0);
```

**这种改变了i状态的****`assert`****代码千万不要写**，**因为会导致debug模式和release模式下的行为不一致**。

# **什么时候使用****`assert`****？**

期望某块逻辑一定要满足某个前置条件，如果不满足就要`crash`，这方便于我们在`debug`模式下调试程序，如果`crash`，方便我们进一步修复问题，使其满足`assert`条件。

比如我们要输出某个字符串，做一次判空其实也是耗费性能的，我们期望传进来的串一定非空，如果是空，那就直接`crash`：

```C++
void func(const char* s) {
  printf("hello %s \n", s);
}
```

# static_assert

上面的`assert`是**运行时断言**，C++11之后还有个**编译时断言：****`static_assert`**，用于在编译时判断某些条件，如果断言失败，则会编译失败，用法和`assert`类似。

```C++
#include <type_traits>
constexpr int k = 120;
int main() { static_assert(k > 200, "k < 200"); }
```

输出：

```C++
test.cc:8:14: error: static_assert failed due to requirement 'k > 200' "k < 200"
int main() { static_assert(k > 200, "k < 200"); }
             ^             ~~~~~~~
1 error generated
```

`static_assert`后面可以跟着一个字符串，在断言失败时，会在控制台输出此字符串，方便提示我们相关信息。

# 练习

- `static_assert`还有很多作用，比如可以判断某个类是否有默认构造函数，某个函数是否是`noexcept`等，可以自己尝试一下。