build:
	mkdir -p target/rescript
	/Users/karolis/projects/zed/target/release/zed-extension --source-dir . --output-dir target/ --scratch-dir target/
	tar -xzf target/archive.tar.gz -C target/rescript
	cp -Rf target/rescript ~/Library/Application\ Support/Zed/extensions/installed/
	tree ~/Library/Application\ Support/Zed/extensions/installed/rescript
