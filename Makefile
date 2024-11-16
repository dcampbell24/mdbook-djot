.PHONY: enable-git-hooks
enable-git-hooks:
	git config --local core.hooksPath .githooks/
