#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
FILES_TXT = ROOT / "files.txt"
REPORT_PATH = ROOT / "docs" / "porting" / "files-coverage-report.md"


def run_br(*args: str) -> str:
    result = subprocess.run(
        ["br", *args],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(
            f"br {' '.join(args)} failed\nstdout={result.stdout}\nstderr={result.stderr}"
        )
    return result.stdout.strip()


def load_checklist_paths() -> list[str]:
    lines = FILES_TXT.read_text(encoding="utf-8").splitlines()
    return [line.strip()[6:] for line in lines if line.startswith("- [")]


def load_coverage_comments() -> dict[str, list[str]]:
    issues = [
        issue
        for issue in json.loads(run_br("list", "--json") or "[]")
        if "coverage" in issue.get("labels", [])
    ]
    mapping: dict[str, list[str]] = {}
    for issue in issues:
        comments = json.loads(run_br("comments", "list", issue["id"], "--json") or "[]")
        matched: list[str] = []
        for comment in comments:
            text = comment.get("text", "")
            if "# Coverage manifest" not in text:
                continue
            collect = False
            for line in text.splitlines():
                if line.strip() == "## Matched checklist rows":
                    collect = True
                    continue
                if not collect:
                    continue
                if line.startswith("- `") and line.endswith("`"):
                    matched.append(line[3:-1])
        if matched:
            mapping[issue["title"]] = matched
    return mapping


def language_of(path: str) -> str:
    if path.startswith("references/typescript/"):
        return "typescript"
    if path.startswith("references/python/"):
        return "python"
    if path.startswith("references/rust/"):
        return "rust"
    return "other"


def build_report() -> str:
    checklist = load_checklist_paths()
    covered_by_issue = load_coverage_comments()
    implementation_only = {
        title: rows
        for title, rows in covered_by_issue.items()
        if not title.startswith(
            (
                "Checklist coverage:",
                "Coverage audit:",
                "Polish pass:",
            )
        )
    }

    covered: set[str] = set()
    duplicate_counter: Counter[str] = Counter()
    for rows in covered_by_issue.values():
        covered.update(rows)
        duplicate_counter.update(rows)

    impl_covered: set[str] = set()
    impl_duplicate_counter: Counter[str] = Counter()
    for rows in implementation_only.values():
        impl_covered.update(rows)
        impl_duplicate_counter.update(rows)

    uncovered = [path for path in checklist if path not in covered]
    impl_uncovered = [path for path in checklist if path not in impl_covered]
    covered_once = [path for path, count in duplicate_counter.items() if count == 1]
    covered_multi = [path for path, count in duplicate_counter.items() if count > 1]
    impl_covered_once = [
        path for path, count in impl_duplicate_counter.items() if count == 1
    ]
    impl_covered_multi = [
        path for path, count in impl_duplicate_counter.items() if count > 1
    ]

    by_language = Counter(language_of(path) for path in checklist)
    uncovered_by_language = Counter(language_of(path) for path in uncovered)
    impl_uncovered_by_language = Counter(language_of(path) for path in impl_uncovered)

    lines = [
        "# Files Coverage Report",
        "",
        "Generated from `files.txt` plus `Coverage manifest` bead comments.",
        "",
        f"- Total checklist rows: `{len(checklist)}`",
        f"- Covered rows: `{len(covered)}`",
        f"- Covered exactly once: `{len(covered_once)}`",
        f"- Covered by multiple beads: `{len(covered_multi)}`",
        f"- Uncovered rows: `{len(uncovered)}`",
        "",
        "## Implementation Coverage",
        "",
        "This section excludes umbrella coverage beads and counts only concrete coverage beads.",
        "",
        f"- Implementation-covered rows: `{len(impl_covered)}`",
        f"- Implementation-covered exactly once: `{len(impl_covered_once)}`",
        f"- Implementation-covered by multiple beads: `{len(impl_covered_multi)}`",
        f"- Implementation-uncovered rows: `{len(impl_uncovered)}`",
        "",
        "## By Language",
        "",
        "| Language | Total | Uncovered | Implementation-Uncovered |",
        "| --- | ---: | ---: | ---: |",
    ]

    for lang in ("typescript", "python", "rust", "other"):
        total = by_language.get(lang, 0)
        if total == 0:
            continue
        lines.append(
            f"| `{lang}` | {total} | {uncovered_by_language.get(lang, 0)} | {impl_uncovered_by_language.get(lang, 0)} |"
        )

    lines.extend(
        [
            "",
            "## Coverage Beads",
            "",
            f"- Coverage bead count with manifests: `{len(covered_by_issue)}`",
            "",
        ]
    )
    for title, rows in sorted(covered_by_issue.items()):
        lines.append(f"- `{title}` -> `{len(rows)}` rows")

    lines.extend(["", "## Uncovered Rows", ""])
    if uncovered:
        for path in uncovered:
            lines.append(f"- `{path}`")
    else:
        lines.append("- none")

    lines.extend(["", "## Multi-Owned Rows", ""])
    if covered_multi:
        for path in sorted(covered_multi):
            lines.append(f"- `{path}` ({duplicate_counter[path]} owners)")
    else:
        lines.append("- none")

    lines.extend(["", "## Implementation-Uncovered Rows", ""])
    if impl_uncovered:
        for path in impl_uncovered:
            lines.append(f"- `{path}`")
    else:
        lines.append("- none")

    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--write-report", action="store_true")
    args = parser.parse_args()

    report = build_report()
    print(report, end="")

    if args.write_report:
        REPORT_PATH.parent.mkdir(parents=True, exist_ok=True)
        REPORT_PATH.write_text(report, encoding="utf-8")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
