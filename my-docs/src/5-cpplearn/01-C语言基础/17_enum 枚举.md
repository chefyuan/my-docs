枚举由一组命名的整型常量组成。枚举为开发人员提供了一种更直观易读的方式来表示一组相关的常量。

**示例代码**：

```C++
#include <iostream>

enum Color {
    RED,
    GREEN,
    BLUE
};

int main() {
    Color favoriteColor = GREEN;

    if (favoriteColor == RED) {
        std::cout << "Your favorite color is red." << std::endl;
    } else if (favoriteColor == GREEN) {
        std::cout << "Your favorite color is green." << std::endl;
    } else if (favoriteColor == BLUE) {
        std::cout << "Your favorite color is blue." << std::endl;
    }

    // 注意：默认情况下，枚举的第一个成员的值为0，后续成员的值依次递增1（除非显式指定）

    return 0;
}
```

这里我们定义了一个`Color`枚举，它包含三个成员：`RED`、`GREEN`和`BLUE`。然后，我们创建了一个`Color`类型的变量`favoriteColor`，并将其设置为`GREEN。`

`favoriteColor`的值既可以为`RED`，也可以为`GREEN`或者`BLUE`。当然，也可以为其它整数值，那就属于骚操作了，可能会出现bug哦。

下面详细介绍。

# 枚举的定义

枚举类型通过`enum`关键字定义，后面跟着枚举类型的名称以及用大括号`{}`括起来的一组枚举常量。每个枚举常量可以用一个标识符来表示，也可以为它们指定一个整数值。如果没有指定值，那么默认从0开始递增。

**基本语法**：

```C
enum 枚举名 {枚举元素1, 枚举元素2, ...};
```

**示例**：

```C
enum DAY {MON = 1, TUE, WED, THU, FRI, SAT, SUN};
```

在这个例子中，`MON`被显式地赋值为1，因此`TUE`的值为2，`WED`的值为3，依此类推。

# 枚举的使用

1. **定义枚举变量**

定义枚举变量的方式有三种：

- 先定义枚举类型，再定义枚举变量：

  - ```C
    enum DAY {MON = 1, TUE, WED, THU, FRI, SAT, SUN};
    enum DAY day;
    ```

- 定义枚举类型的同时定义枚举变量：

  - ```C
    enum DAY {MON = 1, TUE, WED, THU, FRI, SAT, SUN} day;
    ```

- 省略枚举名称，直接定义枚举变量（**不推荐，因为会降低代码的可读性**）：

  - ```C
    enum {MON = 1, TUE, WED, THU, FRI, SAT, SUN} day;
    ```

1. **为枚举元素赋值**

枚举元素的值可以是整数，也可以是其他枚举元素的值。如果未显式赋值，则默认从0开始递增。

```C
enum season {spring, summer = 3, autumn, winter};
```

在这个例子中，`spring`的值为0，`summer`的值为3，`autumn`的值为4（因为`summer`之后自动加1），`winter`的值为5。

1. **枚举在switch语句中的使用**

枚举类型非常适合在`switch`语句中使用，因为它提供了一组有限的、命名的常量。

```C
#include <stdio.h>

enum color {red = 1, green, blue};

int main() {
    enum color favorite_color;
    printf("请输入你喜欢的颜色: (1. red, 2. green, 3. blue): ");
    scanf("%d", &favorite_color); // 注意：这里使用%d读取枚举值，因为枚举在底层是整数

    switch (favorite_color) {
        case red:
            printf("你喜欢的颜色是红色\n");
            break;
        case green:
            printf("你喜欢的颜色是绿色\n");
            break;
        case blue:
            printf("你喜欢的颜色是蓝色\n");
            break;
        default:
            printf("你没有选择你喜欢的颜色\n");
    }

    return 0;
}
```

# 枚举的内存表示

在C语言中，枚举类型通常被当作`int`或`unsigned int`类型来处理。

也就是说，枚举变量在内存中占用的空间与`int`或`unsigned int`相同。

**示例**：

```C
#include <stdio.h>

enum DAY {MON = 1, TUE, WED, THU, FRI, SAT, SUN};

int main() {
    enum DAY day = WED;
    printf("Size of enum DAY: %zu bytes\n", sizeof(enum DAY));
    printf("Value of day (WED): %d\n", day);

    return 0;
}
```

输出：

```Plain
Size of enum DAY: 4 bytes
Value of day (WED): 3
```

# 枚举与整数的相互转换

由于枚举在底层是整数，因此可以很容易地在枚举和整数之间进行转换。

1. **整数转换为枚举**

```C
enum DAY day = (enum DAY)3; // 将整数3转换为枚举DAY的WED
```

1. **枚举转换为整数**

```C
int value = day; // 将枚举变量day的值转换为整数
```

# 高级用法

1. **枚举的位域表示**

在某些情况下，为了节省内存，可以使用位域来表示枚举值。

```C
#include <stdio.h>

enum FLAG {OFF = 0, ON = 1};

struct {
    enum FLAG flag1 : 1;
    enum FLAG flag2 : 1;
    enum FLAG flag3 : 1;
} flags;

int main() {
    flags.flag1 = ON;
    flags.flag2 = OFF;
    flags.flag3 = ON;

    printf("flag1: %d\n", flags.flag1);
    printf("flag2: %d\n", flags.flag2);
    printf("flag3: %d\n", flags.flag3);

    return 0;
}
```

1. **枚举的字符串表示**

为了更方便地调试和打印枚举值，可以定义一个函数将枚举值转换为字符串。

```C
#include <stdio.h>

enum DAY {MON = 1, TUE, WED, THU, FRI, SAT, SUN};

const char* dayToString(enum DAY day) {
    switch (day) {
        case MON: return "Monday";
        case TUE: return "Tuesday";
        case WED: return "Wednesday";
        case THU: return "Thursday";
        case FRI: return "Friday";
        case SAT: return "Saturday";
        case SUN: return "Sunday";
        default: return "Unknown";
    }
}

int main() {
    enum DAY day = WED;
    printf("The day is: %s\n", dayToString(day));

    return 0;
}
```

1. **枚举的遍历**

虽然C语言标准没有提供直接遍历枚举类型的方法，但在某些情况下，如果枚举值是连续的，可以通过整数循环来间接遍历枚举值。

```C
#include <stdio.h>

enum DAY {MON = 1, TUE, WED, THU, FRI, SAT, SUN};

int main() {
    for (int i = MON; i <= SUN; i++) {
        printf("Enum value: %d\n", i);
        // 注意：这里只是打印了枚举的整数值，并没有直接处理枚举类型
    }

    return 0;
}
```

然而，这种方法并不适用于枚举值不连续的情况。对于不连续的枚举值，需要手动处理每个枚举元素。

# 练习

1. 定义一个`enum`类型表示一周的七天。编写一个程序，允许用户输入一个数字（1-7），然后输出对应的星期几名称。