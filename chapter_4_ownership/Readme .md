# 所有权说明
栈和堆的区别
栈像叠盘子一样, 当增加更多盘子时, 把它们放在上面, 取走盘子时, 只能从最上面取走. 栈的大小是固定且已知的, 不会增长或缩小. 

所以当需要一个大小未知的或者可变大小的数据时, 就需要使用堆.使用堆的话, 需要先请求一定大小的空间, 操作系统会在堆上找到一块足够大的空间, 并标记为已使用, 然后返回一个指向这块内存的指针. 这个过程称为在堆上分配内存. 因为数据的指针是一个固定大小的值, 所以可以把指针放在栈上, 但是数据本身放在堆上. 想象去餐馆吃饭, 和服务员说明有几个人,服务员会找一个尽可能大的空桌子并把你领过去, 如果需要有人来晚了, 他们也可以通过询问来找到你们的位置.

入栈比在堆上分配内存要快, 因为入栈无需为存储数据搜索内存空间, 新位置永远在栈顶, 相比之下, 在堆上分配内存需要工作, 因为需要在堆上找到一块足够大的空间来存放数据, 并接着做一些记录为下一次分配做准备.

访问堆上的数据比访问栈上的数据慢, 因为要通过指针访问. 处理器在内存中跳转越少就越快, 栈是连续的(缓存命中), 堆是不连续的(缓存未命中). 继续类比, 一个服务员在餐馆处理多个桌子的订单, 他如果从桌子A听完订单再去桌子B听, 会比较快, 但如果从桌子A听一个菜, 然后去桌子B听一个菜, 再回到桌子A听第二个菜, 这样会比较慢. 同样的原理, 处理器在内存中跳转越少就越快.

跟踪哪部分代码正在使用堆上的哪些数据, 最大程度减少堆上的重复数据的数量, 以及清理不再使用的数据确保不会耗尽空间, 就是所有权系统的工作.


# 所有权规则
    1. Rust 中的每一个值都有一个 所有者 (owner)。
    2. 值在任一时刻有且只有一个所有者。
    3. 当所有者 (变量)离开作用域，这个值将被丢弃。

## 变量作用域
变量的作用域是程序中变量有效的区域. 作用域是通过花括号来定义的. 一个变量的作用域从声明它的地方开始, 一直持续到当前作用域结束. 例如:
```rust
    {                      // s 在这里无效，它尚未声明
        let s = "hello";   // 从此处起，s 是有效的

        // 使用 s
    }                      // 此作用域已结束，s 不再有效
```
换句话说，这里有两个重要的时间点：

- 当 s 进入作用域 时，它就是有效的。
- 这一直持续到它 离开作用域 为止。

### String 类型
目前为止，变量是否有效与作用域的关系跟其他编程语言是类似的。
字符串的类型有两种, 字符串字面值和 String 类型. 字符串字面值是固定的, 在编译时就知道了, 但是并不适合所有场景, 比如想获取用户的字符串时. 所以 Rust 标准库提供了 String 类型, 这个类型被分配在堆上, 所以可以存储一个大小未知的文本. 使用from函数将字符串字面值转换为 String 类型.

```rust
    let s = String::from("hello");
```

这两个::是运算符, 允许将特定的 `from` 函数置于 String 类型的命名空间 (namespace) 下, 而不需要使用类似 string_from 这样的函数名. 这个之后在 `方法语法 (Method Syntax)` 部分会讲到这个语法, 在 `路径用于引用模块树中的项` 中会讲到命名空间.
**可以**修改此类字符串:
```rust
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{s}"); // 将打印 `hello, world!`
```
字符串字面值和String的内存处理方式是不同的, 字符串字面值是硬编码在程序中的, 所以是不可变的.
而String类型, 是在堆上分配的, 意味着:
- 必须在运行时向memory allocator请求内存
- 需要一个处理完String时将内存返回给memory allocator的方法

第一部分由String::from函数处理, 第二部分对于不同的语言有不同的处理方式. 有garbage collector的语言, GC会记录并清理不再使用的内存. 而在其他大部分没有GC的语言中, 识别和清理代码需要程序员自己处理. 
```rust
    {
        let s = String::from("hello"); // s 进入作用域

        // 使用 s

    }                                  // 这里, s 离开作用域并调用 `drop` 方法. 内存被释放
```
Rust使用一个叫做drop的特殊函数来处理. 当String离开作用域时, Rust会自动调用drop函数, 释放内存. 这个模式也被称为`资源获取即初始化(RAII)`.

这个模式看起来简单, 但是在复杂的情况下, 代码的行为可能是不可预测的, 比如有多个使用相同内存的变量.

### 变量与数据的交互方式: 移动
在Rust中, 多个变量采用不同的方式与数据交互. 以下是一些例子:
```rust
    let x = 5;
    let y = x;
```
先将5赋值给x, 然后将变量x的值复制并绑定给y, 现在两个变量都是5. 这也是事实发生的, 因为整数是已知且固定大小的, 所以这两个5是存储在栈上的.
现在看下String类型:
```rust
    let s1 = String::from("hello");
    let s2 = s1;
```
这里发生了什么? String由三部分组成, 如下方左侧所示: 一个指向存储字符串内容的内存的指针, 一个长度, 一个容量. 这三部分在栈上存储, 右侧是堆上存储的"hello"的内容.


![Two tables: the first table contains the representation of s1 on the stack, consisting of its length (5), capacity (5), and a pointer to the first value in the second table. The second table contains the representation of the string data on the heap, byte by byte.](asset/trpl04-01-1.svg)

长度表示String的内容使用了多少字节的内存, 容量表示获取了多少字节的内存. 当s1赋值给s2, String的数据被复制了, 这意味着我们复制了指针,长度和容量. 但是, 没有复制堆上的数据, 如下图所示

![Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap.](asset/trpl04-02.svg)

这种复制并没有像下图一样, 将堆上的数据复制一份, 这样会对运行性能造成非常大的影响

![Four tables: two tables representing the stack data for s1 and s2,
and each points to its own copy of string data on the heap.](asset/trpl04-03.svg)

之前提到变量离开作用域会自动调用drop函数, 现在两个数据的指针都指向了一个位置, 这样就会有一个问题, `s1`和`s2`都会尝试释放相同的内存, 这被称作为`二次释放`的错误. 为了避免这个问题, Rust不会复制堆上的数据, 这个操作被称为`移动`. 当s1赋值给s2时, s1的值被移动到了s2, s1不再有效, 这样就不会有两个指针指向同一个内存位置, 也就不会有内存错误.

```rust
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!"); // 这里会报错, s1不再有效
```
这里会报一个编译错误, 因为rust禁止使用无效的引用.
在其他语言中存在`深拷贝`和`浅拷贝`的概念, 深拷贝会复制堆上的数据, 而浅拷贝只会复制栈上的数据, 拷贝指针, 长度和容量但是不复制数据听起来像是浅拷贝. 但是在Rust中, 这被称为移动, 但是Rust中同时将第一个变量无效化了, 所以这个操作被称为移动. 上面的例子就是移动的例子.

![Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap.
Table s1 is grayed out be-cause s1 is no longer valid; only s2 can be used to
access the heap data.](asset/trpl04-04.svg)

这里其实还隐含了一个设计选择, Rust不会自动的创建数据的`深拷贝` .因此, 任何自动的复制可以被认为是性能影响较小的操作.

### 变量与数据的交互方式: 克隆
如果确实需要复制堆上的数据, 可以使用`clone`方法. 这在很多语言中是一个常见的功能.
例如:
```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
```
这里s1和s2都是有效的, 因为s1的数据被复制到了s2, 这里的clone方法是显式的复制, 而不是移动. 但是这种操作会相当的消耗资源.

### 只在栈上的数据: 复制
在Rust中, 栈上的数据可以复制, 这些代码使用了整型并且是有效的. 这种操作被称为`复制`, 例如:
```rust
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
```
这里和之前的似乎有矛盾, 没有调用`clone`方法, 但x依然有效且没有被移动到y. 这是因为类似整型的数据是已知的固定大小, 所以存储在栈上, 且可以快速复制. 
所以这两个5是存储在栈上的且独立的, 他们的值是相同的, 但是他们是两个不同的变量. 这里没有深浅拷贝的区别, 所以调用clone方法也没什么不同.
Rust有一个叫做`Copy` trait的特殊注解, 可以用在类似整型这样的存储在栈上的类型上(之后会详细讲解trait). 如果一个类型实现了`Copy` trait, 一个旧的变量在将其赋值给其他变量后仍然可用. Rust不允许自身或任何部分实现了`Drop` trait的类型使用`Copy` trait. 如果我们对其值离开作用域时需要特殊处理的类型使用`Copy` trait, 编译器会报错. 要学习的如何为自己的类型添加`Copy` 注解以实现改trait, 参考附录中的`Derivable trait`部分.
可以通过查看给定类型的文档来确定是否实现了`Copy` trait. 但作为一个通用的规则, 任何简单标量类型的组合可以是`Copy`的, 不需要分配内存或某种形式资源的类型是`Copy`的. 下面是一些`Copy`的类型:
- 所有整数类型, 比如u32
- 布尔类型, bool
- 所有浮点数类型, 比如f64
- 字符类型, char
- 元组, 当且仅当其包含的类型也都是Copy的时候. 比如(i32, i32)是Copy的, 但(i32, String)不是.

### 所有权与函数
将值传递给函数和给变量赋值类似. 向函数传递值可能会被移动和复制, 就像赋值语句一样, 参考下面的例子:
```rust
    fn main() {
        let s = String::from("hello");  // s 进入作用域

        takes_ownership(s);             // s 的值移动到函数里 ...
                                        // ... 所以到这里不再有效

        let x = 5;                      // x 进入作用域

        makes_copy(x);                  // x 应该移动函数里，
                                        // 但 i32 是 Copy 的，所以在后面可继续使用 x

    } // 这里, x 先离开作用域，然后是 s。但因为 s 的值已被移走，
      // 所以不会有特殊操作

    fn takes_ownership(some_string: String) { // some_string 进入作用域
        println!("{some_string}");
    } // 这里, some_string 离开作用域并调用 `drop` 方法。占用的内存被释放

    fn makes_copy(some_integer: i32) { // some_integer 进入作用域
        println!("{some_integer}");
    } // 这里, some_integer 离开作用域。由于它是 Copy 的，所以不会有特殊操作
```


### 一些中英文翻译

*后进先出* (last in, first out)

*进栈* (pushing onto the stack)

*出栈* (popping off the stack)

*内存分配器* (memory allocator)

*指针* (pointer)

*在堆上分配内存* (allocating on the heap)

*垃圾回收器* (garbage collector)

*资源获取即初始化* (Resource Acquisition Is Initialization, RAII)

*二次释放* (double free)

*深拷贝* (deep copy)

*浅拷贝* (shallow copy)
