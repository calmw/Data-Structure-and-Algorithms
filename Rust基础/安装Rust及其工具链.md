#### 安装Rust及其工具链

- Ubuntu22.04及其以上版本自带Rust，可以直接使用
- Rust工具链包括编译器rustc、项目管理工具cargo、工具链管理工具rustup、文档管理工具rustdoc、格式化工具rustfmt和调试工具rust-gdb
- 对简单项目可以使用rustc来编译，但涉及大项目时就需要使用cargo工具来管理，其内部也是使用rustc来编译的。