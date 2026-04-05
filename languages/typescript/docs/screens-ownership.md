# TypeScript screen ownership

The archived screen files under bead `aicd-3ix.4.15.1` are interactive Ink/React shells from the original TypeScript product and are not preserved as one-for-one shipped runtime screens in the current AICD TypeScript proving slice.

The shipped TypeScript pack keeps the owned runtime boundary under `languages/typescript/runtime/` and summary/config entry surfaces rather than the archived full-screen UI flow. These `src/screens/*` files therefore need explicit archive-only or future feature-pack disposition instead of being implied as part of the shipped default runtime.

## Ownership and disposition

- `references/typescript/src/screens/Doctor.tsx` — archive-only diagnostics screen retained for source fidelity; the shipped proving slice exposes doctor/runtime validation through summary and runtime-owned validation surfaces rather than this archived interactive Ink screen.
- `references/typescript/src/screens/ResumeConversation.tsx` — archive-only session-resume screen retained for source fidelity; the shipped proving slice preserves session/runtime ownership under `languages/typescript/runtime/` without shipping the archived interactive resume picker UI.
- `references/typescript/src/screens/REPL.tsx` — archive-only monolithic REPL screen retained for source fidelity; its behavior is decomposed conceptually across shipped runtime entrypoint, engine, registry, context, and state boundaries rather than preserved as a direct screen module.

## Shipped-language-pack rule

This subset is complete when each archived screen row has an explicit archive-only or shipped-owner rationale. These archived full-screen UI shells must not be mistaken for already-shipped TypeScript runtime modules just because the proving slice preserves related runtime behavior elsewhere.
