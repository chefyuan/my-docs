这里介绍一下`raw string literal`，它中文名我也不知道怎么翻译更好，但在描述**字符串常量**时，它非常方便且安全。

它的形式大概是这样：

```C++
R"(HelloWorld)";
```

示例代码：

```C++
std::cout << "hello\n World \n meow" << std::endl;
std::cout << R"(hello\n World \n meow)" << std::endl;
```

它们的输出如下：

```C++
hello
 World
 meow
hello\n World \n meow
```

通过它的输出你大体应该也能猜到`raw string literal`的作用，你传进去的是什么字符串，它就会输出什么字符串。

而以前使用普通字符串，遇到`\n \t \`等都需要多加个`\`来转义才可以，如果遇到复杂字符串，加那么多转义，又难看，又容易出错（我们想要的是`\n`，但如果没有转义，它却自动变成了换行）。

所以在`C++`中，建议使用`raw string literal`来表示字符串常量。