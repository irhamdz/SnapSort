# Frontend Conventions — SnapSort

## Component structure
- One component per file, PascalCase filename
- Co-locate component-specific types in the same file
- Keep components under 200 lines; extract sub-components if larger

## State management (Zustand)
- One store slice per domain: `useGalleryStore`, `useBatchStore`, `useSettingsStore`
- Never put derived/computed data in the store — use selectors
- Batch Selection Set is transient: lives in `useBatchStore`, cleared on navigation

## Tailwind CSS v4
- Use Tailwind utility classes only — no CSS modules, no inline styles
- Animation: use `transition-*` utilities; prefer `fade-out` for card removal
- Dark mode via Tailwind's `dark:` prefix

## Tauri IPC
- All `invoke()` calls go through typed wrappers in `src/api/`
- Never call `invoke()` directly in components

## Key UI patterns
- Gallery is virtualized — only visible thumbnails rendered (react-virtual or similar)
- Batch Action Bar slides up from bottom with CSS transform
- Detail panel slides in from right (300ms ease-out)
- Toast notifications: bottom-right, 4s auto-dismiss, stackable, Undo toasts 5s
- Keyboard shortcuts: ⌘K/Ctrl+K search, Esc close/exit, ←/→ navigate, ⌘A/Ctrl+A select all
