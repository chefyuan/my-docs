`static` 是 C++ 中一个非常重要的关键字，它在不同的上下文中有不同的作用。

# 静态局部变量

在函数内部定义的局部变量可以使用 `static` 关键字来声明为静态局部变量。静态局部变量的生命周期贯穿整个程序运行期间，但作用域仍然局限于定义它的函数内部。

```C++
void func() {
    static int count = 0;  // 静态局部变量
    count++;
    std::cout << "Count: " << count << std::endl;
}

int main() {
    func();  // 输出: Count: 1
    func();  // 输出: Count: 2
    func();  // 输出: Count: 3
    return 0;
}
```

上面的例子中，`count` 是一个静态局部变量。每次调用 `func()` 时，`count` 的值都会保留，而不是像普通局部变量那样在函数调用结束后被销毁。

# 静态全局变量

在全局作用域中，`static` 关键字可以用来限制变量的链接性、可见性。

静态全局变量只能在定义它的文件中访问，其他文件无法访问该变量。

```C++
// file1.cpp
static int globalVar = 42;  // 静态全局变量

// file2.cpp
extern int globalVar;  // 错误: globalVar 在 file2.cpp 中不可见
```

`globalVar` 是一个静态全局变量，只能在 `file1.cpp` 中访问。其他文件无法通过 `extern` 关键字来引用它。

# 静态成员变量

在类中，`static` 关键字可以用来声明静态成员变量。

**静态成员变量属于类本身，而不是类的某个实例**。所有类的实例共享同一个静态成员变量。

```C++
class MyClass {
public:
    static int staticVar;  // 静态成员变量声明
};

int MyClass::staticVar = 0;  // 静态成员变量定义

int main() {
    MyClass obj1;
    MyClass obj2;

    obj1.staticVar = 10;
    std::cout << "obj2.staticVar: " << obj2.staticVar << std::endl;  // 输出: 10

    MyClass::staticVar = 20;
    std::cout << "obj1.staticVar: " << obj1.staticVar << std::endl;  // 输出: 20

    return 0;
}
```

`staticVar` 是一个静态成员变量，`obj1` 和 `obj2` 共享同一个 `staticVar`。通过类名或类的实例都可以访问静态成员变量。

# 静态成员函数

在类中，`static` 关键字还可以用来声明静态成员函数。静态成员函数属于类本身，而不是类的某个实例。静态成员函数不能访问类的非静态成员变量和非静态成员函数，因为它们没有 `this` 指针。

```C++
class MyClass {
public:
    static void staticFunc() {  // 静态成员函数
        std::cout << "Static function called." << std::endl;
    }
};

int main() {
    MyClass::staticFunc();  // 通过类名调用静态成员函数
    return 0;
}
```

# 静态局部变量与线程安全

在多线程环境中，静态局部变量的初始化是线程安全的。C++11 标准规定，静态局部变量的初始化只会发生一次，并且在多线程环境下，初始化过程是线程安全的。

```C++
void func() {
    static MyClass obj;  // 线程安全的初始化
    // ...
}
```

在这个例子中，`obj` 是一个静态局部变量，它的初始化在多线程环境下是线程安全的。

正是因为这个特性，我们经常使用static局部变量来构造单例：

```C++
T& Instance() {
    static T t;
    return t;
}
```

# 静态断言

C++11 引入了 `static_assert`，它可以在编译时进行断言检查。

如果断言失败，编译将失败并输出指定的错误信息。

```C++
static_assert(sizeof(int) == 4, "int must be 4 bytes");
```

`static_assert` 检查 `int` 类型的大小是否为 4 字节。如果不是，编译将失败并输出错误信息 `"int must be 4 bytes"`。

# 修饰普通函数

`static` 关键字不仅可以用于变量和类的成员，还可以用于修饰**普通函数**。当 `static` 用于修饰普通函数时，它的作用是**限制函数的链接性**，使得该函数仅在定义它的文件中可见，其他文件无法访问该函数。

当一个普通函数被 `static` 修饰时，它的符号（函数名）不会被导出到全局符号表中，因此其他文件无法通过 `extern` 声明来引用该函数。这种特性可以用于隐藏函数的实现细节，避免命名冲突，或者限制函数的作用域。

```C++
// file1.cpp
static void hiddenFunction() {  // static 修饰的普通函数
    std::cout << "This function is hidden from other files." << std::endl;
}

void publicFunction() {
    hiddenFunction();  // 可以在本文件中调用
}

// file2.cpp
extern void hiddenFunction();  // 错误: hiddenFunction 在 file2.cpp 中不可见

void anotherFunction() {
    hiddenFunction();  // 错误: hiddenFunction 在 file2.cpp 中不可见
}
```

在上面的例子中：

- `hiddenFunction` 被 `static` 修饰，因此它的作用域仅限于 `file1.cpp` 文件。
- `publicFunction` 是一个普通函数，可以在其他文件中通过 `extern` 声明来调用。
- 在 `file2.cpp` 中，尝试调用 `hiddenFunction` 会导致编译错误，因为 `hiddenFunction` 的符号被隐藏了。

## 为什么需要隐藏符号？

- **避免命名冲突**：在大型项目中，可能会有多个文件定义同名的函数。如果这些函数是全局可见的，链接时会发生冲突。使用 `static` 修饰函数可以将其作用域限制在当前文件，避免冲突。
- **封装实现细节**：某些函数可能是某个模块的内部实现细节，不希望被其他模块直接调用。通过 `static` 修饰，可以将这些函数隐藏起来，只暴露必要的接口。
- **优化链接性能**：隐藏符号可以减少全局符号表的体积，从而加快链接过程。

## `static` 函数与匿名命名空间的对比

在 C++ 中，除了使用 `static` 修饰函数来隐藏符号外，还可以使用**匿名命名空间**来实现类似的效果。

```C++
// file1.cpp
namespace {  // 匿名命名空间
    void hiddenFunction() {
        std::cout << "This function is hidden from other files." << std::endl;
    }
}

void publicFunction() {
    hiddenFunction();  // 可以在本文件中调用
}

// file2.cpp
extern void hiddenFunction();  // 错误: hiddenFunction 在 file2.cpp 中不可见
```

匿名命名空间的作用与 `static` 修饰函数类似，都是将符号的作用域限制在当前文件中。它们的区别在于：

- `static` 是 C 语言风格的用法，而匿名命名空间是 C++ 特有的特性。
- 匿名命名空间可以隐藏类、变量、函数等多种符号，而 `static` 只能用于函数和变量。
- 匿名命名空间更符合 C++ 的现代编程风格，推荐优先使用。

# 总结

- **静态局部变量**：在函数内部定义，生命周期贯穿整个程序运行期间，但作用域局限于函数内部。
- **静态全局变量**：在全局作用域中定义，链接性受限，只能在定义它的文件中访问。
- **静态成员变量**：属于类本身，所有类的实例共享同一个静态成员变量。
- **静态成员函数**：属于类本身，不能访问非静态成员变量和非静态成员函数。
- **线程安全的静态局部变量初始化**：在多线程环境下，静态局部变量的初始化是线程安全的。
- **静态断言**：在编译时进行断言检查，用于确保某些条件在编译时成立。
- **修饰普通函数**：可以隐藏符号，不被全局可见。