# 什么时候使用`volatile`关键字？

直接看下面代码：

```C++
int a = 100;

while (a == 100) { 
    // code
}
```

这段程序编译时，如果编译器发现程序始终没有企图改变a的值，那它可能就会优化这段代码，变成`while(true)`的死循环使得程序执行的更快，然而这种优化有时候会变成过度优化，编译器有时候可能没有意识到程序会改变a的值，却做了这种优化导致程序没有产生预期的行为。

这里为了产生预期的行为，需要阻止编译器做这种优化，可以使用`volatile`关键字修饰。

```C++
volatile int a = 100;
```

`volatile`关键字和`const`关键字相对应，`const`关键字告诉编译器其修饰的变量是只读的，编译器根据只读属性做一些操作，而`volatile`关键字告诉编译器其修饰的变量是易变的，同理编译器根据易变属性也会做一些操作。它会确保修饰的变量每次都读操作都从内存里读取，每次写操作都将值写到内存里。

`volatile`关键字就是给编译器做个提示，告诉编译器不要对修饰的变量做过度的优化，提示编译器该变量的值可能会以其它形式被改变。

# **`volatile`****修饰结构体时，结构体的成员也是****`volatile`****的吗**

```C++
struct A {
    int data;
};
volatile A a;
const A b;
```

答案是结构体内所有的都是`volatile`，引用c++标准里的一句话：

> [Note: volatile is a hint to the implementation to avoid aggressive optimization involving the object 
>
> because the value of the object might be changed by means undetectable by an implementation. 
>
> See 1.9 for detailed semantics. In general, the semantics of volatile are intended to be the same 
>
> in C + + as they are in C. ]

这里大体可以理解为一个对象是`volatile`，那对象里所有的成员也都是`volatile`。其实`const`和`volatile`可以理解为是硬币的两面，我们经常听到看到传说中的CV修饰词就是`const`和`volatile`关键字。

# **volatile可以保证原子性吗**

想必大家都知道答案，`volatile`只保证内存可见性，不能保证操作是原子的，拿i++举例：

```C++
volatile int i = 0;
i++; // i = i + 1
```

`i++ `相当于`i=i+1`，而`i=i+1`其实可以分解为好几步：

- 先读取`i`的值到`tmp`
- 增加`tmp`的值
- 把`tmp`的值写回到`i`的地址里

而`volatile`只能保证内存可见，可以理解为上述三步中的每一步都是原子的，但是三步合起来却不一定是原子的，因为在多线程中三步中间可能插入一些其它操作改变了预期的行为，所以**volatile不能用在多线程中**，多线程中的原子操作还是需要使用`atomic`。

**tips：**`volatile`不能解决多线程安全问题，针对特种内存才需要使用`volatile`，它和`atomic`的特点如下：

- `std::atomic`用于多线程访问的数据，且不用互斥量，用于并发编程中
- `volatile`用于读写操作不可以被优化掉的内存，用于特种内存中