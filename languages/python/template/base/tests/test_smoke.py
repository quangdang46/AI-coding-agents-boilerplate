from src.__PACKAGE_NAME__.app import main


def test_main() -> None:
    assert main() == "__PROJECT_NAME__ session loop completed"
