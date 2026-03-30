# Design system

Visual and implementation standards for **all** server-rendered web UI: marketing pages, logged-in flows, settings, and admin. The living guide is at [`/docs/design-system`](/docs/design-system) (MiniJinja + CSS demo).

## Why this exists

- One brutalist, token-based look across the product.
- Less drift (inline styles, one-off colors, softer patterns).
- A single reference for refactors and new pages.

## Source of truth

- **Rendered guide:** `src/web/templates/docs/design-system.html` — `GET /docs/design-system`
- **Tokens and baseline CSS:** `src/web/assets/css/style.css`, `common.css`, `layout.css`
- **App-specific rules:** `src/web/assets/css/specifics.css`

## Required standards

### Brand and visual language

- Brutalist baseline: hard edges, explicit borders, geometric structure, no decorative gradients, no rounded corners.
- Prefer high-contrast, token-based colors over ad hoc values.

### Null / unfilled backgrounds

For regions with no solid content yet (map frames, loading shells, empty preview panes), use the checker tokens from `style.css`, not a flat gray block:

- `var(--checker-background)`
- `var(--checker-background-size)` — set both `background` and `background-size` (see `design-system.html` and `.ds-checker-null-demo` in `specifics.css`).

### CSS and templates

- No inline `style=` attributes.
- No embedded `<style>` blocks in templates for page-specific layout.
- Reuse existing classes (`.box`, `.fields-grid`, buttons, etc.) first.
- New styling: one small class → bottom of `specifics.css`; larger feature set → new file imported from `style.css`.

### MiniJinja and handlers

- Templates stay structural; compute display state in Rust handlers.
- Pass simple context; avoid heavy logic in templates.

## When changing templates

1. Remove inline styles and embedded `<style>` tags.
2. Replace literals with design tokens.
3. Normalize controls and blocks to shared patterns.
4. Keep hover, active, and status colors on existing variables.

## Related docs

- [`frontend-development.md`](frontend-development.md)
- [`minijinja-context-patterns.md`](minijinja-context-patterns.md)
- [`src/web/ROUTES.md`](../src/web/ROUTES.md)
