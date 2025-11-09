# TailwindCSS v4 & shadcn/ui Version Compatibility Guide

**CRITICAL:** This document explains breaking changes between TailwindCSS v3 and v4, and how our project uses the latest versions.

**Last Updated:** November 9, 2025  
**Our Versions:** TailwindCSS v4.1.17, shadcn/ui latest

---

## ⚠️ Breaking Changes Summary

### TailwindCSS v4 is NOT Backward Compatible

TailwindCSS v4 represents a **complete rewrite** of the configuration system. Code and documentation for v3 **will not work** with v4.

| Feature | TailwindCSS v3 (OLD) | TailwindCSS v4 (WE USE THIS) |
|---------|----------------------|------------------------------|
| **Configuration** | `tailwind.config.js` file | **CSS-based** (no config file) |
| **CSS Imports** | `@tailwind base;`<br>`@tailwind components;`<br>`@tailwind utilities;` | `@import "tailwindcss";` |
| **Theme Definition** | JavaScript object in config | `@theme {}` blocks in CSS |
| **Color Format** | Hex (`#646cff`), RGB, HSL | **OKLCH** (`oklch(0.985 0 0)`) |
| **CSS Variables** | Manual setup | `:root` and `.dark` pseudo-classes |
| **Vite Plugin** | `@tailwindcss/postcss@3` | `@tailwindcss/vite` |
| **Variable Exposure** | Manual theme extension | `@theme inline {}` directive |
| **Dark Mode** | `darkMode: 'class'` in config | `.dark {}` CSS block |

---

## Why OKLCH Colors?

TailwindCSS v4 uses **OKLCH** color format instead of hex/RGB/HSL:

### Benefits of OKLCH

- **Perceptually uniform:** Colors with same lightness look equally bright
- **Wider gamut:** Access to more vibrant colors (P3 color space)
- **Better interpolation:** Gradients and animations look smoother
- **Predictable lightness:** `oklch(0.5 X Y)` is always 50% brightness

### OKLCH Syntax

```css
oklch(lightness chroma hue)
oklch(lightness chroma hue / alpha)
```

- **Lightness:** 0 (black) to 1 (white) - `0.985` = very light
- **Chroma:** 0 (gray) to ~0.4 (vibrant) - `0` = no color, `0.245` = saturated
- **Hue:** 0-360 degrees - `0`/`360` = red, `120` = green, `240` = blue
- **Alpha:** 0-1 or percentage - `10%` = 10% opacity

### Examples

```css
/* Neutral colors (chroma = 0) */
--background: oklch(1 0 0);           /* Pure white */
--foreground: oklch(0.145 0 0);       /* Very dark gray */
--primary: oklch(0.205 0 0);          /* Dark gray */

/* Colored variables (with chroma) */
--destructive: oklch(0.577 0.245 27.325);  /* Red (hue ~27) */
--chart-1: oklch(0.646 0.222 41.116);      /* Orange */
--chart-2: oklch(0.6 0.118 184.704);       /* Cyan */

/* With transparency */
--border: oklch(1 0 0 / 10%);         /* White with 10% opacity */
--input: oklch(1 0 0 / 15%);          /* White with 15% opacity */
```

### Converting from Hex to OKLCH

**DO NOT manually convert!** Use tools:

1. **shadcn/ui themes:** <https://ui.shadcn.com/themes> (auto-generates OKLCH)
2. **OKLCH Color Picker:** <https://oklch.com>
3. **Browser DevTools:** Modern browsers show OKLCH in color picker

---

## Configuration Files

### ❌ OLD: TailwindCSS v3 Setup

```javascript
// tailwind.config.js (DO NOT USE - v3 only)
export default {
  darkMode: ['class'],
  content: ['./src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        border: 'hsl(var(--border))',
        primary: {
          DEFAULT: 'hsl(var(--primary))',
          foreground: 'hsl(var(--primary-foreground))',
        },
      },
    },
  },
  plugins: [require('tailwindcss-animate')],
};
```

```css
/* src/index.css (v3 style - DO NOT USE) */
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 222.2 84% 4.9%;
    --primary: 222.2 47.4% 11.2%;
  }
  .dark {
    --background: 222.2 84% 4.9%;
    --foreground: 210 40% 98%;
  }
}
```

### ✅ NEW: TailwindCSS v4 Setup (WE USE THIS)

```typescript
// vite.config.ts
import path from 'path';
import tailwindcss from '@tailwindcss/vite';  // v4 Vite plugin
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [
    react(), 
    tailwindcss()  // Add TailwindCSS v4 plugin
  ],
  resolve: {
    alias: { '@': path.resolve(__dirname, './src') },
  },
});
```

```javascript
// postcss.config.js
export default {
  plugins: {
    '@tailwindcss/postcss': {},  // Required for v4
  },
};
```

```css
/* src/index.css - TailwindCSS v4 style */
@import "tailwindcss";  /* Replaces @tailwind directives */

/* Define CSS variables using OKLCH */
:root {
  --radius: 0.625rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.145 0 0);
  --primary: oklch(0.205 0 0);
  --primary-foreground: oklch(0.985 0 0);
  --border: oklch(0.922 0 0);
}

/* Dark mode theme */
.dark {
  --background: oklch(0.145 0 0);
  --foreground: oklch(0.985 0 0);
  --primary: oklch(0.922 0 0);
  --border: oklch(1 0 0 / 10%);
}

/* Expose CSS variables to TailwindCSS */
@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
  --color-border: var(--border);
}
```

---

## shadcn/ui Integration

### shadcn/ui with TailwindCSS v4

shadcn/ui **latest version** is built for TailwindCSS v4 and uses:

- **OKLCH colors only** (no hex/RGB/HSL in new themes)
- **CSS variable theming** (no utility class fallback)
- **@theme inline** directive for variable exposure
- **components.json** configuration file

### Installation (v4 Compatible)

```bash
# Initialize shadcn/ui (creates components.json)
npx shadcn@latest init

# Follow prompts:
# - Style: Default or New York
# - Base color: Neutral, Slate, Zinc, etc. (all use OKLCH)
# - CSS variables: Yes (required for v4)
# - TypeScript: Yes
# - Path aliases: @/* → ./src/*

# Add components
npx shadcn@latest add button input card form
```

### components.json Example

```json
{
  "style": "default",
  "rsc": false,
  "tailwind": {
    "config": "",
    "css": "src/index.css",
    "baseColor": "neutral",
    "cssVariables": true
  },
  "aliases": {
    "components": "@/components",
    "utils": "@/lib/utils",
    "ui": "@/components/ui",
    "lib": "@/lib",
    "hooks": "@/hooks"
  },
  "iconLibrary": "lucide"
}
```

---

## Common Migration Issues

### Issue 1: "tailwind.config.js not found"

**Cause:** Using v3 documentation or old tutorials  
**Solution:** TailwindCSS v4 **does not use** `tailwind.config.js` - configure in CSS files

### Issue 2: "@tailwind base not working"

**Cause:** Using v3 CSS import syntax  
**Solution:** Replace with `@import "tailwindcss";`

### Issue 3: "hsl(var(--primary)) colors not working"

**Cause:** Using HSL format with OKLCH variables  
**Solution:** Use OKLCH format in CSS variables, reference directly in Tailwind classes

```css
/* ❌ OLD (v3 with HSL) */
--primary: 222.2 47.4% 11.2%;
@theme { --color-primary: hsl(var(--primary)); }

/* ✅ NEW (v4 with OKLCH) */
--primary: oklch(0.205 0 0);
@theme inline { --color-primary: var(--primary); }
```

### Issue 4: "Module '@tailwindcss/postcss' not found"

**Cause:** Installed wrong PostCSS plugin  
**Solution:** Use `@tailwindcss/vite` for Vite projects (not `@tailwindcss/postcss`)

```bash
# Wrong (for non-Vite projects)
npm install -D @tailwindcss/postcss

# Correct (for Vite projects)
npm install -D @tailwindcss/vite
```

### Issue 5: "Colors look different after migration"

**Cause:** OKLCH is perceptually uniform (HSL is not)  
**Solution:** Use shadcn themes generator or OKLCH color picker to find equivalent colors

---

## Official Resources

### TailwindCSS v4

- **Documentation:** <https://tailwindcss.com/docs>
- **Blog (v4 announcement):** <https://tailwindcss.com/blog/tailwindcss-v4-alpha>
- **GitHub Discussions:** <https://github.com/tailwindlabs/tailwindcss/discussions>

### shadcn/ui

- **Vite Installation:** <https://ui.shadcn.com/docs/installation/vite>
- **Theming Guide:** <https://ui.shadcn.com/docs/theming>
- **Themes Generator:** <https://ui.shadcn.com/themes>
- **Components:** <https://ui.shadcn.com/docs/components>

### OKLCH Color Tools

- **OKLCH Color Picker:** <https://oklch.com>
- **Color Converter:** <https://oklch.evilmartians.io>
- **MDN OKLCH Reference:** <https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/oklch>

---

## DO NOT Use These Resources

- ❌ **TailwindCSS v3 documentation** (completely different config system)
- ❌ **Stack Overflow answers from 2023 or earlier** (likely v3)
- ❌ **YouTube tutorials using `tailwind.config.js`** (outdated)
- ❌ **ChatGPT/AI-generated code without verification** (trained on v3)
- ❌ **shadcn/ui examples with hex colors** (old theme system)

---

## Verification Checklist

Before assuming your setup is correct, verify:

- [ ] No `tailwind.config.js` file exists
- [ ] `vite.config.ts` imports and uses `@tailwindcss/vite`
- [ ] `postcss.config.js` uses `@tailwindcss/postcss`
- [ ] `src/index.css` starts with `@import "tailwindcss";`
- [ ] All CSS variables use OKLCH format (not hex/RGB/HSL)
- [ ] `@theme inline {}` block exists to expose variables
- [ ] `components.json` has `"cssVariables": true`
- [ ] Dark mode uses `.dark {}` CSS block (not config)

---

## Questions?

1. **Check official v4 docs first:** <https://tailwindcss.com/docs>
2. **Check shadcn/ui docs:** <https://ui.shadcn.com/docs>
3. **Search GitHub Discussions:** Filter by "TailwindCSS v4"
4. **Verify you're not using v3 solutions**

**Last Resort:** If stuck, ask in project chat with:

- Your exact error message
- Your `vite.config.ts` and `src/index.css` files
- Confirmation you checked official v4 documentation
