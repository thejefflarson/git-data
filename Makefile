install:
	mkdir -p /usr/local/libexec/
	cp libexec/* /usr/local/libexec/
	ln -s /usr/local/libexec/git-data /usr/local/bin/git-data
