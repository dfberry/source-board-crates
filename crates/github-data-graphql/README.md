# CLI: get repo data

This CLI takes in the org, repo, and PAT and returns JSON of the repo data. 

## Usage

1. Put PAT (personal access token to GitHub) in `.env`

    ```
    PAT='ghp_...'
```

1. Create bash script to use PAT.

    ```
    #!/bin/bash
    #chmod +x cargo_run.sh

    if [ -f .env ]; then
    export $(cat .env | xargs)
    fi

    cargo run -- --org MicrosoftDocs --repo node-essentials --pat $PAT
    ```

1. Make bash script executable.

    ```bash
    chmod +x cargo_run.sh
    ```

1. Run script. 

    ```bash
    ./cargo_run.sh
    ```

1. Get results.

    ```json
    {
        "createdAt": "2020-07-08T14:51:56Z",
        "description": "MS Learn Node Essentials sample code",
        "diskUsage": 932,
        "name": "node-essentials",
        "pushedAt": "2024-12-02T13:57:36Z",
        "updatedAt": "2024-11-30T05:39:29Z",
        "url": "https://github.com/MicrosoftDocs/node-essentials",
        "watchersCount": 19
    }
    ```