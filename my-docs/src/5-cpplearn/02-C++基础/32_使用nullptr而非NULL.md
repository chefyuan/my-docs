大家应该都知道这个观点，就是在**C++中要使用****`nullptr`****，而不要用****`NULL`****。**

那为什么要这样做，这篇文章简单介绍一下。

`nullptr`的使用代码如下：

```C++
int *ptr = nullptr;
```

同样是表示空指针，之前`NULL`使用的好好的，为什么要引入`nullptr`？`nullptr`和`NULL`又有什么区别呢？

# **NULL究竟是什么？**

```C++
#ifndef NULL
#ifdef __cplusplus
#define NULL 0
#else
#define NULL ((void *)0)
#endif
#endif
```

从源码中可以知道，在C中`NULL`是`((void *)0)`指针，在C++中`NULL`却是个`整数0`。

为什么同样是`NULL`，在`C`和`C++`中却有不同的定义呢？

`C++`中有一个很特别的规定就是0既表示**整形常量**也用来表示**空指针常量**。

`C++03`标准中提过：

> A null pointer constant is an integral constant expression (expr.const) rvalue of integer type that evaluates to zero. A null pointer constant can be converted to a pointer type; the result is the null pointer value of that type and is distinguishable from every other value of pointer to object or pointer to function type. Two null pointer values of the same type shall compare equal. The conversion of a null pointer constant to a pointer to cv-qualified type is a single conversion, and not the sequence of a pointer conversion followed by a qualification conversion (conv.qual).

主要规定空指针常量需要被转化成指针类型，同时这个转化为指针类型的值还不能和其它的对象指针或者函数指针的值相同。两个空指针常量的值还需要相等。

而`C99`标准中：

> An integer constant expression with the value 0, or such an expression cast to type void *, is called a null pointer constant.[55] If a null pointer constant is converted to a pointer type, the resulting pointer, called a null pointer, is guaranteed to compare unequal to a pointer to any object or function.

主要就是说C中的空指针常量是整型`0`被强转成了`void*`，这就可以确保这个空指针的值与其它对象或函数指针值不相等。

这里`C++`中的`NULL`如果和C语言一样也是`(void *)0`指针，而`C++`却又不允许`void*`隐式转换成其它指针类型，那还怎么用`NULL`来表示空指针呢，岂不是尴尬了。

下面代码编译会报错：

```C++
#include <iostream>

int main() {
 int *a = (void *)0;
 return 0;
}
error : cannot initialize a variable of type 'int *' with an rvalue of type 'void *'
```

# **为什么要引入nullptr？**

一个原因是可以让`整型0`放下重担，`0`只做一件事情，它只是一个整数类型`0`，没有任何其它语义，空指针的活就安排给其它员工，这个员工就是`nullptr`关键字。

先看一段代码：

```C++
void func(char*) { cout << "char*"; }
void func(int) { cout << "int"; }

int main() {
 func(NULL);  // 编译失败 error: call of overloaded ‘func(NULL)’ is ambiguous
 func(nullptr); // char*
 return 0;
}
```

另一个原因是在`C++`的函数重载中，传入`NULL`会导致编译失败，所以需要引入`nullptr`，使用`nullptr`可以解决函数重载中的参数匹配问题。

这里可以总结三点：

- 使用`nullptr`可以不用担心整型和指针类型的重载，不会产生二义性导致编译失败。
- `0`和空指针分别表示不同的含义，使用`nullptr`可以更好的支持模板编程。
- 使用`nullptr`使代码更安全，让用户编写代码更清晰直观。

`C++`之父也说过：

> Should I use NULL or 0?
>
> In C++, the definition of NULL is 0, so there is only an aesthetic difference. I prefer to avoid macros, so I use 0. Another problem with NULL is that people sometimes mistakenly believe that it is different from 0 and/or not an integer. In pre-standard code, NULL was/is sometimes defined to something unsuitable and therefore had/has to be avoided. That's less common these days.If you have to name the null pointer, call it nullptr; that's what it's called in C++11. Then, "nullptr" will be a keyword.

`NULL`其实就是一个宏，对于宏，`C++`之父一直推崇尽量避免使用它，在实际编程中，可以减少宏的使用，直接使用0。`Bjarne Stroustrup`语录也给出了解释。所以在`C++`中，完全可以抛弃掉`NULL`，不得已可以`使用0`替代。

**既然****`NULL`****就是****`0`****，那为什么不直接使用****`0`****，而搞出来一个****`NULL`****呢？**

因为需要为空指针常量起一个名字，更清晰的表明它表达的是什么含义，就像3.1415926为什么要用π表示一样，尽管宏一直是被各方吐槽的，但为了有名字在当时C++也只能这样，这也是`NULL`宏面世的唯一一个理由。

所以如果编译器支持`nullptr`，一定要使用`nullptr`。

# **空指针应该有什么特性吗？**

1. 它应该有一个自己的名字，它应该是一个保留关键字。
2. 空指针不能够被用于算数表达式中，不能被赋值给整型，也不能用于和指针类型外的类型做比较。
3. 空指针可以被转化成任何指针类型，不能被转换成指针类型外的任何类型。

你有没有想过，`nullptr`为什么可以转换成`int*`， `float*`等？

看下它可能的实现：

```C++
struct nullptr_t {
 void operator&() const = delete; // Can't take address of nullptr

 template <class T>
 inline operator T*() const {
  return 0;
 }

 template <class C, class T>
 inline operator T C::*() const {
  return 0;
 }
};

nullptr_t nullptr;
```

过实现了转换运算符，`nullptr`可以转换成`int*`等，同时，为什么不能对`nullptr`取址？因为它的取址操作被`delete`修饰了。

# **使用nullptr还有什么好处呢？**

可以用于抛出异常。

`nullptr`是有类型的：

```C++
typdef decltype(nullptr) nullptr_t;
```

当空指针用`nullptr`表示时，空指针就终于有类型了，当有异常需要抛出时，就可以抛出`nullptr`。

```C++
int main() {
 try {
  ... throw nullptr;
 } catch (nullptr_t) {
  ...
 }
}
```

之后使用它的类型`nullptr_t`捕获，这里如果`throw NULL`，那用什么类型去`catch`呢？用`int`类型来`catch`？是不是有点别扭。所以能用`nullptr`就一定要用`nullptr`代替`NULL`。