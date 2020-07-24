#!/bin/sh
echo $1 > /Users/amagex/projects/dapkit/vscode/x0a-dapkit/bin/test.txt
DAPKIT_PATH = $(dirname ${BASH_SOURCE[0]})
$DAPKIT_PATH/dapkit vscode --json $1