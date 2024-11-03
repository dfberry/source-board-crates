#!/bin/bash

# Variables
ACR_NAME="crnsxqg7ibsurvu" # Replace with your ACR name
SP_NAME="rrg-postgres-api-cicd-crnsxqg7ibsurvu"   # Replace with your desired Service Principal name
RESOURCE_GROUP="rg-postgres-api" # Replace with your resource group name
KEYVAULT= "kkv-nsxqg7ibsurvu" # Replace with your Key Vault name

# Log in to Azure
az login

# Get the ACR registry ID
ACR_REGISTRY_ID=$(az acr show --name $ACR_NAME --resource-group $RESOURCE_GROUP --query id --output tsv)

# Create the Service Principal
SP=$(az ad sp create-for-rbac --name http://$SP_NAME --scopes $ACR_REGISTRY_ID --role acrpush acrpull)

# Assign 'acrpush' role to the Service Principal
az role assignment create --assignee $(echo $SP | jq -r '.client_id') --role acrpush --scope $ACR_REGISTRY_ID

# Assign 'acrpull' role to the Service Principal
az role assignment create --assignee $(echo $SP | jq -r '.client_id') --role acrpull --scope $ACR_REGISTRY_ID

# Output the Service Principal details
echo "Service Principal created with the following details:"
echo $SP

# Extract and display individual values
CLIENT_ID=$(echo $SP | jq -r '.client_id')
CLIENT_SECRET=$(echo $SP | jq -r '.client_secret')
TENANT_ID=$(echo $SP | jq -r '.tenant_id')

echo "Client ID: $CLIENT_ID"
echo "Client Secret: $CLIENT_SECRET"
echo "Tenant ID: $TENANT_ID"