# TypeScript keybindings ownership

The archived TypeScript `src/keybindings/*` package in bead `aicd-3ix.4.15.1.4` is not preserved as a one-for-one shipped runtime package in the current AICD TypeScript proving slice.

The shipped TypeScript language pack currently exposes only a summary-level `keybindings` command through `languages/typescript/runtime/registry/coreCommands.ts`; it does not ship the archived keybinding provider, parser, schema, and UI hooks as a dedicated `languages/typescript/runtime/keybindings/` boundary. These archived files therefore remain archive-only unless a future UI/input feature extraction explicitly adopts them.

## Ownership and disposition

- `references/typescript/src/keybindings/match.ts` — archive-only keybinding match helper retained for source fidelity; the shipped proving slice does not expose a dedicated keybinding matcher module.
- `references/typescript/src/keybindings/loadUserBindings.ts` — archive-only user-binding loader retained for source fidelity; user-editable keybinding loading is not part of the shipped TypeScript proving slice.
- `references/typescript/src/keybindings/shortcutFormat.ts` — archive-only shortcut-formatting helper retained for source fidelity; the shipped proving slice does not expose this formatting helper as a runtime module.
- `references/typescript/src/keybindings/defaultBindings.ts` — archive-only default-binding table retained for source fidelity; the proving slice does not ship the archived keybinding defaults package.
- `references/typescript/src/keybindings/reservedShortcuts.ts` — archive-only reserved-shortcut helper retained for source fidelity; snapshot-era shortcut reservation logic is not part of the shipped proving slice.
- `references/typescript/src/keybindings/useKeybinding.ts` — archive-only React keybinding hook retained for source fidelity; interactive keybinding UI/runtime hooks remain outside the shipped proving slice.
- `references/typescript/src/keybindings/KeybindingContext.tsx` — archive-only keybinding context provider retained for source fidelity; the shipped TypeScript pack does not expose this UI/runtime context boundary.
- `references/typescript/src/keybindings/validate.ts` — archive-only keybinding validation helper retained for source fidelity; the shipped proving slice does not ship the archived validation module.
- `references/typescript/src/keybindings/KeybindingProviderSetup.tsx` — archive-only keybinding provider setup retained for source fidelity; provider wiring for interactive keybinding UI remains outside the shipped proving slice.
- `references/typescript/src/keybindings/useShortcutDisplay.ts` — archive-only shortcut-display hook retained for source fidelity; interactive keybinding display helpers are not part of the shipped proving slice.
- `references/typescript/src/keybindings/schema.ts` — archive-only keybinding schema helper retained for source fidelity; the shipped proving slice does not expose a dedicated keybinding schema runtime boundary.
- `references/typescript/src/keybindings/template.ts` — archive-only keybinding template helper retained for source fidelity; template-generation for shortcut data is outside the shipped proving slice.
- `references/typescript/src/keybindings/parser.ts` — archive-only keybinding parser retained for source fidelity; the shipped TypeScript proving slice does not ship the archived parser as a standalone runtime module.
- `references/typescript/src/keybindings/resolver.ts` — archive-only keybinding resolver retained for source fidelity; keybinding resolution remains outside the shipped proving slice.

## Shipped-language-pack rule

This subset is complete when each archived keybinding row has an explicit archive-only or shipped-owner rationale. These snapshot-era keybinding files must not be mistaken for already-shipped TypeScript runtime modules.
