关于`C++11`新特性，最先提到的肯定是类型推导，`C++11`引入了`auto`和`decltype`关键字，使用它们可以在编译期就推导出变量或者表达式的类型，方便开发者编码的同时也简化了代码。

#### auto关键字

`auto`关键字允许编译器在编译期间自动推导出变量的类型。这一特性极大地简化了代码，尤其是当变量类型较为复杂或冗长时。

##### 基本用法

```C++
auto a = 10;       // a是int型
int i = 10;
auto b = i;        // b是int型
auto d = 2.0;      // d是double型
```

在上述代码中，编译器根据等号右边的值自动推导出变量的类型。

##### 推导规则

直接看代码：

```C++
void func(auto value) {} // error，auto不能用作函数参数

class A {
    auto a = 1; // error，在类中auto不能用作非静态成员变量
    static auto b = 1; // error，这里与auto无关，正常static int b = 1也不可以
    static const auto int c = 1; // ok
};

void func2() {
    int a[10] = {0};
    auto b = a; // ok
    auto c[10] = a; // error，auto不能定义数组，可以定义指针
    vector<int> d;
    vector<auto> f = d; // error，auto无法推导出模板参数
}
```

- **直接初始化**：`auto`变量必须在声明时立即初始化，否则编译器无法推导出其类型。

```C++
auto e; // error，使用auto必须马上初始化
```

- **一行多个变量**：当在一行中声明多个`auto`变量时，它们必须能够从一个共同的初始化表达式中推导出相同类型。

```C++
int i = 10;
auto a = i, &b = i, *c = &i; // a是int，b是int的引用，c是int的指针
auto d = 0, f = 1.0; // error，0和1.0类型不同，编译器懵了，无法推导
```

- **忽略cv属性和引用**：在推导类型时，`auto`会忽略等号右边的`const`、`volatile`（cv）属性和引用类型，除非显式地声明为引用或指针。

```C++
const int x = 10;
auto y = x; // y是int，忽略了const属性
auto& z = x; // z是const int&，保留了引用和const属性
```

- **限制**：
  - `auto`的使用必须马上初始化，否则无法推导出类型
  - `auto`在一行定义多个变量时，各个变量的推导不能产生二义性，否则编译失败
  - `auto`不能用作函数参数
  - 在类中`auto`不能用作非静态成员变量
  - `auto`不能定义数组，可以定义指针
  - `auto`无法推导出模板参数

```C++
int i = 0;
auto *a = &i; // a是int*
auto &b = i; // b是int&
auto c = b; // c是int，忽略了引用

const auto d = i; // d是const int
auto e = d; // e是int

const auto& f = e; // f是const int&
auto &g = f; // g是const int&
```

##### 扩展应用

尤其是在处理复杂类型时，如`lambda`表达式、智能指针、容器迭代器等，`auto`应用广泛。

```C++
auto lambda = []() { return 42; }; // lambda表达式类型复杂，使用auto简化
auto ptr = std::make_shared<int>(10); // 智能指针类型复杂，使用auto简化
std::vector<int> vec = {1, 2, 3};
auto it = vec.begin(); // 容器迭代器类型复杂，使用auto简化
```

##### 什么时候使用auto？

这里没有绝对答案，只能说一下我自己的理解，个人认为在不影响代码代码可读性的前提下尽可能使用auto是蛮好的，复杂类型就使用`auto`，`int`、`double`这种就没有必要使用`auto`了吧，看下面这段代码：

```C++
auto func = [&] {
    cout << "xxx";
}; // 对于func你难道不使用auto吗，反正我是不关心lambda表达式究竟是什么类型。

auto asyncfunc = std::async(std::launch::async, func);
// 对于asyncfunc你难道不使用auto吗，我是懒得写std::futurexxx等代码，而且我也记不住它返回的究竟是什么...
```

#### decltype关键字

与`auto`不同，`decltype`用于推导表达式的类型，而不是变量的类型。它主要用于编译器分析表达式的类型，而表达式本身不会进行运算。

##### 基本用法

```C++
int func() { return 0; }
decltype(func()) i; // i为int类型，与func()返回值类型相同
int x = 0;
decltype(x) y; // y是int类型，与x类型相同
decltype(x + y) z; // z是int类型，与表达式x + y的类型相同
```

##### 推导规则

- **表达式类型**：`decltype(exp)`的类型与`exp`的类型相同，除非`exp`是左值。
- **函数调用**：如果`exp`是函数调用，则`decltype(exp)`的类型与函数返回值的类型相同。
- **左值引用**：如果`exp`是左值，则`decltype(exp)`是`exp`类型的左值引用。这一点与`auto`不同，`auto`会忽略左值引用。

```C++
int a = 0, b = 0;
decltype(a + b) c = 0; // c是int，因为(a+b)返回一个右值
decltype(a += b) d = c; // d是int&，因为(a+=b)返回一个左值
d = 20; // 修改d实际上修改了a的值
```

- **保留cv属性和引用**：与`auto`不同，`decltype`会保留表达式的cv属性和引用类型。

```C++
const int& ref = a;
decltype(ref) e = 20; // e是const int&，保留了const引用属性
```

##### 扩展应用

`decltype`常用于模板编程中，特别是在推导函数返回值类型时。它允许程序员编写更加通用的代码，而无需显式地指定返回类型。

```C++
template<typename T, typename U>
auto add(T t, U u) -> decltype(t + u) {
    return t + u;
}
```

在上述代码中，`add`函数的返回值类型依赖于其参数类型。通过使用`decltype(t + u)`，我们可以让编译器自动推导出返回值的类型，而无需手动指定。

#### `auto`和`decltype`的配合使用

在C++11中，`auto`和`decltype`经常配合使用，特别是在处理复杂类型和模板编程时。它们共同提供了强大的类型推导能力，使得代码更加简洁和灵活。

```C++
std::vector<int> vec = {1, 2, 3, 4, 5};
auto it = vec.begin(); // 使用auto简化迭代器类型声明
decltype(*it) value = *it; // 使用decltype推导迭代器指向的元素的类型
```

在上述代码中，我们首先使用`auto`简化了迭代器类型的声明。然后，我们使用`decltype`推导了迭代器指向的元素的类型，并将其用于声明一个新的变量`value`。

`auto`和`decltype`一般配合使用在推导函数返回值的类型问题上。

看下面这段代码

```C++
template<typename T, typename U>
return_value add(T t, U u) { // t和v类型不确定，无法推导出return_value类型
    return t + u;
}
```

上面代码由于t和u类型不确定，那如何推导出返回值类型呢，我们可能会想到这种

```C++
template<typename T, typename U>
decltype(t + u) add(T t, U u) { // t和u尚未定义
    return t + u;
}
```

这段代码在C++11上是编译不过的，因为在`decltype(t +u)`推导时，t和u尚未定义，就会编译出错，所以有了下面的叫做返回类型后置的配合使用方法：

```C++
template<typename T, typename U>
auto add(T t, U u) -> decltype(t + u) {
    return t + u;
}
```

返回值后置类型语法就是为了解决函数返回值类型依赖于参数但却难以确定返回值类型的问题。

#### 总结

在实际编程中，我们应该根据具体情况合理地使用这两个关键字。对于简单的内置类型（如`int`、`double`等），直接使用具体类型可能更加直观和清晰。然而，对于复杂类型（如lambda表达式、智能指针、容器迭代器等），使用`auto`和`decltype`可以极大地简化代码并提高可读性。

此外，应该注意到`auto`和`decltype`的一些限制和潜在问题。例如，`auto`变量必须在声明时立即初始化，否则编译器无法推导出其类型。同样地，`decltype`在推导左值引用类型时也需要特别注意，因为它会保留表达式的左值引用属性。