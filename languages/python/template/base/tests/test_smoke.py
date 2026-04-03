from src.__PACKAGE_NAME__.app import main


def test_main() -> None:
    assert "ready" in main()
