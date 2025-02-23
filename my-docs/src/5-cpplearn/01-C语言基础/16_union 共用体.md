`union`是一种特殊的数据结构，`union`在相同的内存位置存储不同类型的数据。`union`中的所有成员共享同一块内存空间，因此在任何时候，`union`只能存储其成员中的一个值。

**示例代码**：

```C++
#include <iostream>

union Data {
    int i;
    float f;
    char str[20];
};

int main() {
    Data data;

    // 存储整数
    data.i = 42;
    std::cout << "Integer: " << data.i << std::endl;

    // 存储浮点数（注意：会覆盖之前的整数数据）
    data.f = 3.14f;
    std::cout << "Float: " << data.f << std::endl;

    // 存储字符串（注意：会覆盖之前的浮点数数据）
    strcpy(data.str, "Hello, Union!");
    std::cout << "String: " << data.str << std::endl;

    return 0;
}
```

在这个示例中，我们定义了一个`Data` `union`，它包含三个成员：

- `i`（整型）
- `f`（浮点型）
- `str`（字符数组）

然后演示了如何向`union`中存储不同类型的数据。

请注意，由于`union`的所有成员共享内存，因此同时访问多个成员可能会导致未定义行为。

下面详细介绍。

# 共用体的定义与基本用法

共用体的定义方式与结构体类似，但使用`union`关键字。

共用体可以包含多个成员，但任何时候只能有一个成员带有有效值。

**因为共用体的所有成员共享同一块****内存****区域，当给某个成员赋值时，会覆盖该内存区域中之前存储的数据。**

**定义共用体的语法如下**：

```C
union 共用体名 {
    成员类型1 成员名1;
    成员类型2 成员名2;
    ...
    成员类型n 成员名n;
};
```

其中，`共用体名`是可选的，用于标识该共用体类型。`成员类型`和`成员名`则定义了共用体中包含的成员及其类型。

```C
union Data {
    int i;
    float f;
    char str[20];
};
```

这个例子，定义了一个名为`Data`的共用体，它包含三个成员：一个整型变量`i`，一个浮点型变量`f`，以及一个字符数组`str`。

**使用共用体**：

```C
#include <stdio.h>
#include <string.h>

union Data {
    int i;
    float f;
    char str[20];
};

int main() {
    union Data data;

    // 给整型成员赋值
    data.i = 10;
    printf("data.i: %d\n", data.i);

    // 给浮点型成员赋值（会覆盖之前的整数值）
    data.f = 220.5;
    printf("data.f: %.5f\n", data.f);

    // 给字符数组成员赋值（会覆盖之前的浮点数值）
    strcpy(data.str, "Hello, Union!");
    printf("data.str: %s\n", data.str);

    return 0;
}
```

在这个示例中，我们定义了一个`Data`类型的共用体变量`data`，并分别给它的整型、浮点型和字符数组成员赋值。由于共用体的成员共享同一块内存，因此每次赋值都会覆盖之前存储的数据。

# 共用体的内存占用

共用体占用的内存大小等于其最大成员所占用的内存大小。

因为共用体的所有成员都存储在同一个内存位置，所以只需要足够的空间来存储最大的成员即可。

**示例**：

```C
#include <stdio.h>

union Data {
    int i;
    float f;
    char str[20];
};

int main() {
    printf("Size of union Data: %zu bytes\n", sizeof(union Data));
    return 0;
}
```

在这个示例中，我们打印了`Data`类型共用体的内存大小。由于字符数组`str`是成员中占用空间最大的（20个字节），因此`Data`类型共用体的内存大小也是20个字节。

# 共用体的应用场景

共用体在C语言中有多种应用场景：

1. **节省内存**：

1. 当需要在同一个内存位置存储多种类型的数据时，可以使用共用体来节省内存。

例如，在处理网络通信数据时，可能会根据协议的不同而需要解析不同类型的数据（如整型、浮点型、字符串等），此时可以使用共用体来存储这些数据。

1. **类型转换**：

1. 共用体可以用于在不同类型之间进行转换。例如，在某些情况下，可能需要将一个整型数转换为浮点型数，或者将一个字符数组转换为整型数进行运算。虽然C语言提供了类型转换运算符，但在某些情况下使用共用体可能方便。

# 共用体与结构体的区别

- **内存占用**：结构体中的每个成员都占用独立的内存空间，而共用体中的所有成员共享同一块内存空间。
- **数据访问**：在结构体中，可以同时访问多个成员的值；而在共用体中，每次只能访问一个有效成员的值（即最后一次被赋值的成员）。
- **用途**：结构体通常用于表示具有固定结构和属性的数据记录（如学生信息、图书信息等），而共用体则更多地用于在不同类型的数据之间进行转换或节省内存。

# 共用体与结构体的结合使用

下面展示了如何将共用体与结构体结合使用来存储多种类型的数据和额外的属性信息：

```C
#include <stdio.h>
#include <string.h>

// 定义一个共用体来存储不同类型的数据
union DataValue {
    int intValue;
    float floatValue;
    char strValue[20];
};

// 定义一个结构体来存储数据值和额外的属性信息
struct DataRecord {
    union DataValue value;
    char type; // 用于标识存储的数据类型（如'i'表示整型，'f'表示浮点型，'s'表示字符串）
    char description[50]; // 用于存储数据的描述信息
};

int main() {
    struct DataRecord record;

    // 存储整型数据
    record.value.intValue = 123;
    record.type = 'i';
    strcpy(record.description, "This is an integer value.");
    printf("Record 1: Type = %c, Value = %d, Description = %s\n", record.type, record.value.intValue, record.description);

    // 存储浮点型数据（会覆盖之前的整数值）
    record.value.floatValue = 456.78;
    record.type = 'f';
    strcpy(record.description, "This is a float value.");
    printf("Record 2: Type = %c, Value = %.2f, Description = %s\n", record.type, record.value.floatValue, record.description);

    // 存储字符串数据（会覆盖之前的浮点数值）
    strcpy(record.value.strValue, "Hello, Union!");
    record.type = 's';
    strcpy(record.description, "This is a string value.");
    printf("Record 3: Type = %c, Value = %s, Description = %s\n", record.type, record.value.strValue, record.description);

    return 0;
}
```

在这个示例中，我们定义了一个`DataValue`类型的共用体来存储不同类型的数据（整型、浮点型、字符串），又定义了一个`DataRecord`类型的结构体来存储数据值和额外的属性信息（如数据类型和描述信息）。

# 练习

- 定义一个`union`，包含`int`、`float`和`char`类型的成员。编写一个程序，允许用户选择数据类型（通过输入），然后根据选择输入相应的值，并输出该值。