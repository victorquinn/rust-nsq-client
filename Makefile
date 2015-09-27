watch:
	fswatch -o **/*.rs | xargs -n1 -I{} cargo test
