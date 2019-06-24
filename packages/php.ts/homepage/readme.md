# @nyar/php-homepage

The `@nyar/php` homepage plus integrated user documentation, authored with the VMZ
framework.

- Homepage: `src/pages/index.vmz`
- User docs: `documents/<locale>/*.md`, mounted at `/d/`
- Locales: `en-us` (default) and `zh-hans`

```bash
pnpm install
pnpm dev      # local session
pnpm check    # VMZ checks + strict document checks
pnpm build    # static build (homepage + /d/*)
```

Docs URLs after build:

- `/d/` → redirects to the default locale
- `/d/en-us/`
- `/d/zh-hans/`
