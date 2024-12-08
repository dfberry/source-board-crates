#!/bin/bash
#chmod +x cargo_run.sh

if [ -f .env ]; then
  export $(cat .env | xargs)
fi

cargo run -- --org MicrosoftDocs --repo node-essentials --pat $PAT
#cargo run 