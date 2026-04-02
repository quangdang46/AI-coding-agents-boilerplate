from __future__ import annotations

from .app import main as app_main


def main() -> int:
    print(app_main())
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
