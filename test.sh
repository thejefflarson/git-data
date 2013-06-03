#!/bin/bash -ex
# Any copyright is dedicated to the Public Domain.
# http://creativecommons.org/publicdomain/zero/1.0/
# adapted from git-fat

export GD_DEBUG=1

git init data-test
cd data-test
git data init
git config git-data.origin.url "localhost:/tmp/git-data"
echo '*.data filter=data -crlf' > .gitattributes
git add .gitattributes
git commit -m 'Initial data repository'

echo 'data content a' > a.data
git add a.data
git commit -m 'add a.data'
echo 'data content b' > b.data
git add b.data
git commit -m 'add b.data'
echo 'revise data content a' > a.data
git commit -am 'revise a.data'
git data sync

cd ..
git clone data-test data-test2
cd data-test2
git data init
git config git-data.origin.url "localhost:/tmp/git-data"
git pull
git data sync
cat a.data