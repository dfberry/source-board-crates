#!/bin/bash

# Variables
ACR_NAME="crnsxqg7ibsurvu" # Replace with your ACR name
SP_NAME="rrg-postgres-api-cicd-crnsxqg7ibsurvu"   # Replace with your desired Service Principal name
RESOURCE_GROUP="rg-postgres-api" # Replace with your resource group name
KEY_VAULT_NAME="kv-nsxqg7ibsurvu" # Replace with your Key Vault name
AZURE_SUBSCRIPTION_ID="19016922-4bf5-4c41-9553-8eff5da1500e"

# Log in to Azure
#az login

# Get the ACR registry ID
ACR_REGISTRY_ID=$(az acr show --name $ACR_NAME --resource-group $RESOURCE_GROUP --query id --output tsv)
echo "ACR Registry ID: $ACR_REGISTRY_ID"

# Create the Service Principal1
SP=$(az ad sp create-for-rbac --name http://$SP_NAME --role acrpush --scopes "subscriptions/$AZURE_SUBSCRIPTION_ID/resourceGroups/$RESOURCE_GROUP")
echo "Service Principal: $SP"

ASSIGNEE=$(echo $SP | jq -r '.appId')
echo "ASSIGNEE: $ASSIGNEE"

# Assign 'acrpull' role to the Service Principal
#az role assignment create --assignee $ASSIGNEE --role acrpull --scope "subscriptions/$AZURE_SUBSCRIPTION_ID/resourceGroups/$RESOURCE_GROUP"

# Extract and display individual values
CLIENT_ID=$(echo $SP | jq -r '.appId')
CLIENT_SECRET=$(echo $SP | jq -r '.password')
TENANT_ID=$(echo $SP | jq -r '.tenant')

echo "Client ID: $CLIENT_ID"
echo "Client Secret: $CLIENT_SECRET"
echo "Tenant ID: $TENANT_ID"

# Store the Service Principal credentials in Azure Key Vault
# az keyvault secret set --vault-name $KEY_VAULT_NAME --name "${SP_NAME}-client-id" --value $CLIENT_ID
# az keyvault secret set --vault-name $KEY_VAULT_NAME --name "${SP_NAME}-client-secret" --value $CLIENT_SECRET
# az keyvault secret set --vault-name $KEY_VAULT_NAME --name "${SP_NAME}-tenant-id" --value $TENANT_ID


