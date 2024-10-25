#!/bin/bash

# Variables
KEY_VAULT_NAME="source-board-key"
SECRET_NAME="source-board-crates-json-azd-env-for-resources"

# Retrieve the JSON value from the Azure Key Vault
json_value=$(az keyvault secret show --name $SECRET_NAME --vault-name $KEY_VAULT_NAME --query value -o tsv)

# Parse the JSON and export each property as an environment variable
echo $json_value | jq -r 'to_entries | .[] | "export " + .key + "=\"" + .value + "\""' | while read -r line; do
    eval $line
done

# Verify the environment variables
env | grep -E 'AZURE_|POSTGRES_|SERVICE_WEB_'