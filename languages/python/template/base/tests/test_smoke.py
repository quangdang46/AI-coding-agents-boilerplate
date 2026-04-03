from src.__PACKAGE_NAME__.app import main


def test_main() -> None:
    output = main()
    assert output.startswith("__PROJECT_NAME__ session loop completed ")
    assert "provider=openai" in output
    assert "model=gpt-5.4" in output
    assert "prompt_digest=" in output
    assert "context_digest=" in output
    assert "approval_mode=default" in output
    assert "bash_policy=bash=approval-required" in output
    assert "file_write_policy=file_write=approval-required" in output
