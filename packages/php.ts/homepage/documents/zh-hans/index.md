# PHP 语言服务

`@nyar/php` 通过原生 Node-API 插件把 PHP 解析能力带到 Node.js。它是只读的：
你给出源码文本，它返回语法树、词法单元、诊断与高亮区间。

## 能拿到什么

| 导出 | 返回值 |
|------|--------|
| `parse(source)` | `{ ok, error, diagnostics, root, tokenCount }` |
| `tokens(source)` | 带字节偏移的 `TokenSpan[]` |
| `check(source)` | `string[]` 诊断信息 |
| `highlight(source)` | 关键字/字符串/数字/注释/标识符区间 |
| `version()` | 原生绑定版本 |

## 安装

```bash
npm install @nyar/php
```

匹配当前平台的包会自动安装。

## 下一步

- [安装](./install.md)
- [命令行](./cli.md)
- [API](./api.md)
- [平台与排障](./native.md)
