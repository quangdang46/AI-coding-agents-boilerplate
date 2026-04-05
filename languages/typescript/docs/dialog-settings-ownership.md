# TypeScript dialog and settings component ownership

The archived dialog-heavy and settings-heavy component files in bead `aicd-3ix.4.15.1.1` are not preserved as a one-for-one shipped UI package in the current AICD TypeScript proving slice.

This slice splits into two ownership outcomes:

- onboarding and auth-oriented dialog surfaces are owned conceptually by the optional `oauth-onboarding` feature pack under `languages/typescript/features/oauth-onboarding/`
- settings, validation-dialog, and keybinding-warning UI surfaces remain archive-only because the shipped proving slice exposes those concerns through summary/runtime/config surfaces rather than the archived interactive component tree

## Ownership and disposition

- `references/typescript/src/components/Onboarding.tsx` — owned conceptually by the optional `oauth-onboarding` feature-pack boundary rather than preserved as a direct component port in the proving slice.
- `references/typescript/src/components/ConsoleOAuthFlow.tsx` — owned conceptually by the optional `oauth-onboarding` feature-pack boundary rather than preserved as a direct component port in the proving slice.
- `references/typescript/src/components/ApproveApiKey.tsx` — owned conceptually by the optional `oauth-onboarding` feature-pack boundary rather than preserved as a direct component port in the proving slice.
- `references/typescript/src/components/InvalidConfigDialog.tsx` — archive-only config-dialog component retained for source fidelity; the proving slice uses runtime/config validation surfaces instead of this archived UI dialog.
- `references/typescript/src/components/InvalidSettingsDialog.tsx` — archive-only settings-dialog component retained for source fidelity; the proving slice uses runtime/config validation surfaces instead of this archived UI dialog.
- `references/typescript/src/components/ManagedSettingsSecurityDialog/ManagedSettingsSecurityDialog.tsx` — archive-only managed-settings security dialog retained for source fidelity; managed-settings UI remains outside the shipped proving slice.
- `references/typescript/src/components/ManagedSettingsSecurityDialog/utils.ts` — archive-only managed-settings dialog helper retained for source fidelity; supporting UI helpers for managed-settings security remain outside the shipped proving slice.
- `references/typescript/src/components/Settings/Settings.tsx` — archive-only settings screen retained for source fidelity; the shipped proving slice does not expose the archived settings UI as a runtime boundary.
- `references/typescript/src/components/Settings/Config.tsx` — archive-only settings config panel retained for source fidelity; the proving slice exposes config behavior via runtime/config summaries rather than this archived UI panel.
- `references/typescript/src/components/Settings/Usage.tsx` — archive-only usage panel retained for source fidelity; the proving slice exposes usage behavior through runtime/session summaries rather than this archived UI panel.
- `references/typescript/src/components/Settings/Status.tsx` — archive-only status panel retained for source fidelity; the proving slice exposes status behavior through runtime/session summaries rather than this archived UI panel.
- `references/typescript/src/components/ConfigurableShortcutHint.tsx` — archive-only shortcut-hint component retained for source fidelity; interactive shortcut UI remains outside the shipped proving slice.
- `references/typescript/src/components/KeybindingWarnings.tsx` — archive-only keybinding-warning component retained for source fidelity; keybinding UI warnings remain outside the shipped proving slice.

## Shipped-language-pack rule

This subset is complete when each archived dialog/settings component row has an explicit archive-only or shipped-owner rationale. These snapshot-era UI files must not be mistaken for already-shipped TypeScript runtime modules.
