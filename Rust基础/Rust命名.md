#### Rust命名

- Rust推荐采用驼峰式命名（UpperCamelCase）来表示类级别的内容，而采用蛇形命名（snake case）来描述值级别的内容。  
  Rust中各种值的推荐的命名风格如下所示：

| 项         | 约定                          |
|-----------|-----------------------------|
| 包(create) | snake_case                  |
| 类型        | UpperCamelCase              |
| 特性（Trait） | UpperCamelCase              |
| 枚举        | UpperCamelCase              |
| 函数        | snake_case                  |
| 方法        | snake_case                  |
| 构造函数      | new或者with_more_details      |
| 构造函数      | from_other_type             |
| 宏         | snake_case！                 |
| 局部变量      | snake_case                  |
| 静态变量      | SCREAMING_SNAKE_CASE        |
| 常量        | SCREAMING_SNAKE_CASE        |
| 类型参数      | UpperCamelCase的首字母，如T、U、K、V |
| 生命周期      | lowercase,如‘a、'src、'dest    |

- 在snake_case或SCREAMING_SNAKE_CASE模式下，单词不应由单个字母组成，除非是最后一个单词。
  例如使用btree_map而不使用b_tree_map,使用PI_2而不使用PI2