结构体是一种用户自定义的数据类型，它可以将多个不同类型的数据项组合成一个单一的类型。结构体中的每个数据项被称为成员。

**示例代码**：

```C++
#include <iostream>
#include <string>

struct Person {
    std::string name;
    int age;
    float height;
};

int main() {
    Person person1;
    person1.name = "Alice";
    person1.age = 30;
    person1.height = 5.5f;

    std::cout << "Name: " << person1.name << ", Age: " << person1.age << ", Height: " << person1.height << std::endl;

    return 0;
}
```

这里我们定义了一个`Person`结构体，它包含三个成员：

- `name`（字符串类型）
- `age`（整型）
- `height`（浮点型）

然后，我们创建了一个`Person`类型的变量`person1`，并给它的成员赋值，这就是结构体的常见用法。

下面详细介绍。

# 结构体的定义

结构体定义由`struct`关键字和结构体名组成，结构体名可以根据需要自行定义。结构体内部可以包含多个成员，每个成员都是标准的变量定义，如整型、浮点型、字符型等，也可以是其他结构体类型或指针类型。结构体定义的格式如下：

```C
struct 结构体标签 {
    成员列表;
} 变量列表;
```

- **结构体标签**：用于标识结构体类型，是可选的。如果不指定结构体标签，则该结构体为**匿名结构体**。
- **成员列表**：包含结构体中的多个成员，每个成员之间用分号分隔。
- **变量列表**：在结构体定义末尾，可以指定一个或多个结构体变量。这些变量在定义时就具有结构体类型。

示例：

```C
// 定义一个名为Student的结构体，包含姓名、年龄和学号三个成员
struct Student {
    char name[50];
    int age;
    int student_id;
};

// 使用结构体标签声明结构体变量
struct Student stu1, stu2;

// 或者在定义结构体的同时声明变量
struct {
    char name[50];
    int age;
    int student_id;
} stu3; // stu3是一个匿名结构体的变量
```

# 结构体的初始化

和其他类型的变量一样，结构体变量也可以在定义时指定初始值。

示例：

```C
#include <stdio.h>
#include <string.h>

struct Student {
    char name[50];
    int age;
    int student_id;
};

int main() {
    // 在定义时初始化结构体变量
    struct Student stu = {"Alice", 20, 1001};

    // 或者使用字符串复制函数初始化字符数组类型的成员
    struct Student stu2;
    strcpy(stu2.name, "Bob");
    stu2.age = 22;
    stu2.student_id = 1002;

    printf("Name: %s, Age: %d, Student ID: %d\n", stu.name, stu.age, stu.student_id);
    printf("Name: %s, Age: %d, Student ID: %d\n", stu2.name, stu2.age, stu2.student_id);

    return 0;
}
```

# 结构体的访问

要访问结构体变量的成员，需要使用成员访问运算符（`.`）。

示例：

```C
#include <stdio.h>

struct Student {
    char name[50];
    int age;
    int student_id;
};

int main() {
    struct Student stu = {"Alice", 20, 1001};

    // 访问结构体成员并打印
    printf("Name: %s\n", stu.name);
    printf("Age: %d\n", stu.age);
    printf("Student ID: %d\n", stu.student_id);

    return 0;
}
```

# 结构体作为函数参数

结构体可以作为函数参数进行传递，传参方式与其他类型的变量或指针类似。

传递结构体时，可以选择传递结构体变量的值（值传递）或传递结构体变量的地址（指针传递）。

- **值传递**：函数接收的是结构体变量的副本，对副本的修改不会影响原变量。
- **指针传递**：函数接收的是结构体变量的地址，可以通过指针直接修改原变量。

示例：

```C
#include <stdio.h>
#include <string.h>

struct Student {
    char name[50];
    int age;
    int student_id;
};

// 值传递
void printStudentByVal(struct Student stu) {
    printf("Name: %s, Age: %d, Student ID: %d\n", stu.name, stu.age, stu.student_id);
}

// 指针传递
void printStudentByPtr(struct Student *stu) {
    printf("Name: %s, Age: %d, Student ID: %d\n", stu->name, stu->age, stu->student_id);
}

int main() {
    struct Student stu = {"Alice", 20, 1001};

    // 值传递调用
    printStudentByVal(stu);

    // 指针传递调用
    printStudentByPtr(&stu);

    return 0;
}
```

在上面的代码中，`printStudentByVal`函数通过值传递接收结构体变量，而`printStudentByPtr`函数通过指针传递接收结构体变量的地址。

在指针传递的情况下，我们使用箭头运算符（`->`）来访问结构体成员。

# 结构体的嵌套与指针

结构体成员可以包含其他结构体，也可以包含指向自己结构体类型的指针。这种嵌套和指针的应用可以实现更高级的数据结构，如链表、树等。

- **结构体嵌套**：一个结构体中可以包含另一个结构体作为成员。
- **结构体指针**：一个结构体变量可以包含一个指向自己类型的指针，用于实现链表等数据结构。

示例：

```C
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

// 定义一个名为Person的结构体
struct Person {
    char name[50];
    int age;
};

// 定义一个名为Employee的结构体，包含Person结构体作为成员
struct Employee {
    struct Person personal_info;
    char job_title[50];
};

// 定义一个名为Node的结构体，包含指向自己类型的指针
struct Node {
    int data;
    struct Node *next;
};

int main() {
    // 初始化Employee结构体变量
    struct Employee emp;
    strcpy(emp.personal_info.name, "Alice");
    emp.personal_info.age = 30;
    strcpy(emp.job_title, "Engineer");

    // 打印Employee结构体变量的信息
    printf("Name: %s, Age: %d, Job Title: %s\n", emp.personal_info.name, emp.personal_info.age, emp.job_title);

    // 创建链表节点
    struct Node *head = (struct Node *)malloc(sizeof(struct Node));
    head->data = 1;
    head->next = NULL;

    struct Node *second = (struct Node *)malloc(sizeof(struct Node));
    second->data = 2;
    second->next = NULL;

    head->next = second; // 将第二个节点链接到第一个节点之后

    // 打印链表节点的数据
    struct Node *current = head;
    while (current != NULL) {
        printf("Data: %d\n", current->data);
        current = current->next;
    }

    // 释放链表节点的内存
    free(head);
    free(second);

    return 0;
}
```

在上面的代码中，`Employee`结构体包含了`Person`结构体作为成员，实现了结构体嵌套。而`Node`结构体则包含了指向自己类型的指针，用于实现链表数据结构。

# 练习

- 定义一个表示图书信息的`struct`，包括书名（`char`数组）、作者（`char`数组）、出版年份（`int`）和价格（`float`）。编写一个程序，允许用户输入图书信息，并使用函数输出图书信息。