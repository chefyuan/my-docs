本文主要介绍`namespace`和`using`。

# 什么是`namespace`？

`namespace`是指命名空间，表示某个变量标识符的可见空间，比如下面的代码：

```C++
namespace Meow {
  int k = 100;
}

int main() {
  std::cout << k << std::endl;
}
```

这段代码中在命名空间`Meow`中定义了变量`k`，表示变量`k`只有在命名空间`Meow`中才可见。

所以下面直接访问`k`这种情况，编译器会报错：

```C++
test.cc:9:18: error: use of undeclared identifier 'k'; did you mean 'Meow::k'?
    std::cout << k << std::endl;
                 ^
                 Meow::k
test.cc:5:9: note: 'Meow::k' declared here
    int k = 100;
        ^
1 error generate
```

那怎么才能访问到这个`k`？

需要显式声明使用某个命名空间的`k`，比如：

```C++
std::cout << Meow::k << std::endl;
```

还有一种方法访问到`k`，就是把它整体包含到对应的`namespace`中：

```C++
namespace Meow {
  int k = 100;
  void f() {
    std::cout << k << std::endl;
  }
}
```

等等，还有一种方法，利用`using`，可以提前声明好使用哪个`namespace`：

```C++
using namespace Meow;
void f() {
  std::cout << k << std::endl;
}
```

这段代码就提前声明了使用`Meow`的`namespace`，这样在访问k时就会默认去`Meow`的`namespace`里寻找，编译器也就不会报错了。

**我们经常见到的****`using namespace std`****就是这个作用。**

另外`namespace`可以嵌套，比如：

```C++
namespace A {
  namespace B {
    int k = 100;
  }
}

int main() {
  std::cout << A::B::k;
}
```

也可以这样嵌套：

```C++
namespace A::B {
  int k = 100;
}
```

两种嵌套的使用方法相同。

# **那为什么要引入****`namespace`****？**

在大的项目工程里，有好多个程序员，不可避免的会定义相同名字的函数，比如`Print`就可能会定义好多次，但一个工程中，同名函数定义多次就会报错，所以才引入了`namespace`，把不同模块里的函数用不同的`namespace`包裹起来，就避免了**`multi definition`**的报错。

# using的作用？

前面简单介绍了`using`的使用，其实`using`还有另一个作用，就是起别名：

```C++
using meowint = int;
using meowllong = long long;
using MeowFrameMap = std::map<meowint, Frame>;

meowint a = 1;
meowllong b = 2;
MeowFrameMap m;
```

比如**`using 美国总统 = 拜登`**，就是给拜登起了个别名叫美国总统，以后叫美国总统的时候别人也知道就是叫的拜登。

`using`非常方便，比如有一个很复杂的`std::map<meowint, Frame>`，每次使用`std::map<meowint, Frame>`定义变量就非常麻烦，

而有了`using`，就可以使用`MeowFrameMap`，方便快捷，表示的含义也很清晰。