from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "keybindings-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/keybindings/match.ts",
    "references/typescript/src/keybindings/loadUserBindings.ts",
    "references/typescript/src/keybindings/shortcutFormat.ts",
    "references/typescript/src/keybindings/defaultBindings.ts",
    "references/typescript/src/keybindings/reservedShortcuts.ts",
    "references/typescript/src/keybindings/useKeybinding.ts",
    "references/typescript/src/keybindings/KeybindingContext.tsx",
    "references/typescript/src/keybindings/validate.ts",
    "references/typescript/src/keybindings/KeybindingProviderSetup.tsx",
    "references/typescript/src/keybindings/useShortcutDisplay.ts",
    "references/typescript/src/keybindings/schema.ts",
    "references/typescript/src/keybindings/template.ts",
    "references/typescript/src/keybindings/parser.ts",
    "references/typescript/src/keybindings/resolver.ts",
]


def test_typescript_keybindings_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_keybindings_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 14
