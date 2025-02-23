关于inline，我们直接了解以下几个知识点即可。

# inline是一个请求（而非命令）

`inline`关键字用于向编译器发出一个请求，建议将函数体在每个调用点内联展开。

这意味着编译器在编译过程中，可能会将函数的代码直接插入到调用该函数的地方，而不是通过通常的函数调用机制来执行。

需要注意的是，`inline`只是一个建议，编译器可以选择是否接受这个建议。

编译器可能会基于多种因素（如函数的大小、复杂性、调用频率以及整体代码的优化目标）来决定是否进行内联展开。

# inline函数通常用于小函数

`inline`函数通常用于那些执行速度快且调用频繁的小函数。这些函数通常只有几行代码，并且不包含复杂的控制结构或大量的计算。

通过将这些小函数内联展开，可以减少函数调用的开销（如栈帧的创建和销毁、参数传递等），从而提高程序的执行效率。

对于较大的函数或包含复杂逻辑的函数，内联展开可能会导致代码膨胀，甚至可能降低性能。

# inline函数的定义通常放在头文件中

由于`inline`函数需要在每个调用点展开，因此其定义需要在编译时对每个编译单元可见。这通常意味着`inline`函数的定义应该放在头文件中，而不是源文件（.c）中。这样做可以确保在链接时不会出现重复定义的问题，因为每个编译单元都会包含`inline`函数的定义，并且编译器会处理这些重复的定义（实际上，由于`inline`的特性，编译器会将其视为建议而非强制要求，因此不会有链接时的符号冲突）。

如果`inline`函数在多个编译单元中被引用，并且没有通过`static`关键字限制其链接性，那么在某些编译器和链接器实现中可能会遇到链接错误。为了避免这种情况，通常建议将`inline`函数声明为`static`，从而限制其在单个编译单元内的可见性。

推荐阅读：https://stackoverflow.com/questions/31108159/what-is-the-use-of-the-inline-keyword-in-c

# inline函数不能包含复杂的控制结构

`inline`函数通常应该避免包含复杂的控制结构，如循环和递归。这是因为内联展开这些结构可能会导致代码膨胀，增加程序的内存占用，并且可能不会带来性能上的提升。

特别是递归函数，由于递归调用本身的开销和栈空间的使用，内联展开通常是不合适的。

对于包含复杂控制结构的函数，即使它们很小，编译器也可能选择不进行内联展开。

# 编译器可能忽略inline请求

编译器可能会忽略`inline`请求，特别是在以下情况下：

- **函数体较大**：如果函数体包含大量代码，编译器可能会认为内联展开会导致代码膨胀，从而选择不进行内联。
- **包含复杂逻辑**：如果函数包含复杂的控制结构、大量的计算或条件分支，编译器可能会认为内联展开不会带来性能上的优势，甚至可能降低性能。
- **优化级别**：编译器的优化级别也会影响其是否接受`inline`请求。在较低的优化级别下，编译器可能会更加保守地选择不进行内联展开。
- **链接时优化**：在某些情况下，即使函数在源代码中被标记为`inline`，编译器也可能在链接时决定不进行内联展开。这取决于编译器的链接时优化能力和策略。

# 验证是否inline

看这段代码：

```C++
#include <stdio.h>
#include <time.h>
#include <stdint.h>

// 定义一个inline函数
inline int add_inline(int a, int b) {
    return a + b;
}

// 定义一个普通函数
int add_normal(int a, int b) {
    return a + b;
}

int main() {
    const double N = 1000000000000000000; // 函数调用次数
    clock_t start, end;
    double cpu_time_used;

    // 测试普通函数
    start = clock();
    for(double i = 0; i < N; i += 0.0000000001) {
        add_normal(i, i);
    }
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("Normal function took %f seconds to execute \n", cpu_time_used);

    // 测试inline函数
    start = clock();
    for(double i = 0; i < N; i += 0.0000000001) {
        add_inline(i, i);
    }
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("Inline function took %f seconds to execute \n", cpu_time_used);

    

    return 0;
}
```

汇编代码：

```C++
add_normal(int, int):
 lea    eax,[rdi+rsi*1]
 ret
 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
main:
 push   rbx
 call   1040 <clock@plt>
 mov    rbx,rax
 call   1040 <clock@plt>
 sub    rax,rbx
 cvtsi2sd xmm0,rax
 divsd  xmm0,QWORD PTR [rip+0xe8a]        # 2008 <_IO_stdin_used+0x8>
 lea    rdi,[rip+0xe8b]        # 2010 <_IO_stdin_used+0x10>
 mov    al,0x1
 call   1030 <printf@plt>
 call   1040 <clock@plt>
 mov    rbx,rax
 call   1040 <clock@plt>
 sub    rax,rbx
 xorps  xmm0,xmm0
 cvtsi2sd xmm0,rax
 divsd  xmm0,QWORD PTR [rip+0xe5c]        # 2008 <_IO_stdin_used+0x8>
 lea    rdi,[rip+0xe8a]        # 203d <_IO_stdin_used+0x3d>
 mov    al,0x1
 call   1030 <printf@plt>
 xor    eax,eax
 pop    rbx
 ret
```

代码中有`add_normal`的标签，却没有`add_line`的标签。

我们可以通过汇编代码查看是否被`inline`，汇编代码中，被内联的函数不会有函数的标签，普通的函数会有函数标签。具体详见：https://godbolt.org/z/j1GrjW85q （注意，要使用-O2编译）。

# 推荐阅读

- https://stackoverflow.com/questions/10631283/how-will-i-know-whether-inline-function-is-actually-replaced-at-the-place-where
- https://stackoverflow.com/questions/31108159/what-is-the-use-of-the-inline-keyword-in-c