from __future__ import annotations

import argparse
from pathlib import Path

from ..commands.doctor import run_doctor
from ..commands.feature import run_feature_add, run_feature_remove
from ..config.loader import load_config



def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description='AICD Python boilerplate runtime utilities')
    subparsers = parser.add_subparsers(dest='command', required=True)

    doctor_parser = subparsers.add_parser('doctor', help='validate a generated Python boilerplate project')
    doctor_parser.add_argument('--project', default='.')

    config_parser = subparsers.add_parser('show-config', help='print parsed boilerplate config')
    config_parser.add_argument('--project', default='.')

    feature_parser = subparsers.add_parser('feature', help='apply or remove a local feature pack')
    feature_subparsers = feature_parser.add_subparsers(dest='feature_action', required=True)
    feature_add = feature_subparsers.add_parser('add', help='add a feature pack')
    feature_add.add_argument('feature_id')
    feature_add.add_argument('--project', default='.')
    feature_remove = feature_subparsers.add_parser('remove', help='remove a feature pack')
    feature_remove.add_argument('feature_id')
    feature_remove.add_argument('--project', default='.')

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    project = Path(args.project).resolve()
    if args.command == 'doctor':
        report = run_doctor(project)
        print(report.as_text())
        return 0 if report.ok else 1
    if args.command == 'show-config':
        print(load_config(project))
        return 0
    if args.command == 'feature':
        feature_project = Path(getattr(args, 'project', '.')).resolve()
        if args.feature_action == 'add':
            print(run_feature_add(feature_project, args.feature_id))
            return 0
        if args.feature_action == 'remove':
            print(run_feature_remove(feature_project, args.feature_id))
            return 0
    parser.error(f'unknown command: {args.command}')
    return 2


if __name__ == '__main__':
    raise SystemExit(main())
