#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

GOOGLEAPIS_HASH=e8d2996cc44d20d430413f60e88c45fdccc20e4c

if [[ ! ${PWD##*/} = "proto" ]]; then
  echo "Error: Run me from the proto folder"
  exit 1;
fi

curl -L -o googleapis.tar.gz https://github.com/googleapis/googleapis/archive/$GOOGLEAPIS_HASH.tar.gz
tar xzf googleapis.tar.gz
rm -r googleapis.tar.gz
rm -Rf googleapis
mv googleapis-$GOOGLEAPIS_HASH googleapis
ls googleapis/ | egrep -v "(google|LICENSE|\.\.)" | xargs -I{} rm -R googleapis/{}
ls googleapis/google | egrep -v "(firestore|api|type|rpc|\.\.)" | xargs -I{} rm -R googleapis/google/{}

cat > googleapis/.gitignore << EOF

*

!*/

!*.proto
EOF
