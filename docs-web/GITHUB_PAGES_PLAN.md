# GitHub Pages Plan for Cortex

## 📍 Estado Actual

**Repo:** https://github.com/southwest-ai-labs/cortex

Ya tenemos:
- ✅ `docs-web/` - Proyecto Astro + Starlight
- ✅ `docs/` - Documentación adicional

---

## 🔧 Setup Requerido

### 1. Habilitar GitHub Pages

```bash
# Ir a: Settings > Pages > Build and deployment
# Source: GitHub Actions
```

### 2. Workflow de GitHub Actions

Crear `.github/workflows/docs.yml`:

```yaml
name: Documentation

on:
  push:
    branches: [main]
    paths:
      - 'docs-web/**'
      - '.github/workflows/docs.yml'

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: 'npm'
          cache-dependency-path: docs-web/package-lock.json

      - name: Install dependencies
        run: |
          cd docs-web
          npm ci

      - name: Build
        run: |
          cd docs-web
          npm run build

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: docs-web/dist

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

---

## 🌐 URL Resultado

```
https://southwest-ai-labs.github.io/cortex/
# o
https://cortex.ai (si configuramos custom domain)
```

---

## 📊 Estructura de docs-web/

```
docs-web/
├── astro.config.mjs      # Config Starlight
├── src/
│   ├── content/docs/
│   │   ├── guides/
│   │   │   ├── intro.md
│   │   │   ├── installation.md
│   │   │   └── quick-start.md
│   │   ├── architecture/
│   │   │   └── overview.md
│   │   ├── modules/
│   │   │   └── memory.md
│   │   ├── reference/
│   │   │   └── api.md
│   │   └── testing/
│   │       └── overview.md
│   └── styles/
│       └── custom.css
└── public/
```

---

## 🎨 Custom Domain (Opcional)

Para usar `docs.cortex.ai`:

1. **GitHub Pages:**
   - Settings > Pages > Custom domain
   - Agregar `docs.cortex.ai`

2. **DNS (Cloudflare):**
   - CNAME: `docs` → `southwest-ai-labs.github.io`

3. **SSL:** Automático con GitHub

---

## 📋 Pasos para Activar

### Opción 1: GitHub Actions (Recomendado)

1. Crear `.github/workflows/docs.yml`
2. Pushear al repo
3. Ir a Settings > Pages
4. Seleccionar "GitHub Actions"
5. Done! ✅

### Opción 2: Branch-based

1. Crear branch `gh-pages`
2. Build local: `npm run build`
3. Push `dist/` a `gh-pages`
4. Settings > Pages > Branch: gh-pages

---

## 🔄 Mantenimiento

- **Actualizar docs:** Editar archivos en `docs-web/src/content/docs/`
- **Build automático:** GitHub Actions corre en cada push a `main`
- **Preview local:** `npm run dev` en `docs-web/`

---

## 📈 SEO & Metadata

Agregar a `astro.config.mjs`:

```javascript
export default defineConfig({
  site: 'https://cortex.ai',
  integrations: [starlight({
    title: 'Cortex Docs',
    description: 'Cognitive Memory for AI Agents',
    social: {
      github: 'https://github.com/southwest-ai-labs/cortex',
    },
    // ...
  })],
});
```

---

## 🎯 Checklist

- [ ] Crear workflow `.github/workflows/docs.yml`
- [ ] Habilitar Pages en Settings
- [ ] Configurar custom domain (opcional)
- [ ] Actualizar README con URL
- [ ] Testing: `npm run build` local

---

*Plan creado: 2026-03-11*
