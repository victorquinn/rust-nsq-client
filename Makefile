test:
	fswatch -o src | xargs -n1 -I{} cargo test
