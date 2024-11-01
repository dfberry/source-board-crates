#!/bin/bash

# BUILD AND DEPLOY

DOTENV_PATH=".env"

# Load environment variables from .env file into the script's environment
if [ -f $DOTENV_PATH ]; then
  set -a
  source $DOTENV_PATH
  set +a
else
  echo "Error: .env file not found at $DOTENV_PATH"
  exit 1
fi

# Debug: Display all environment variables loaded from .env
echo "Loaded environment variables from $DOTENV_PATH:"
while IFS='=' read -r key value; do
  echo "$key=$value"
done < $DOTENV_PATH

az containerapp update \
  --subscription $AZ_SUB_ID \
  --name $AZ_APP_NAME \
  --resource-group $AZ_RG \
  --set-env-vars PORT=secretref:port

az containerapp update \
  --subscription $AZ_SUB_ID \
  --name $AZ_APP_NAME \
  --resource-group $AZ_RG \
  --set-env-vars DATABASE_URL=secretref:database-url

# # Deploy image
az containerapp up \
  --name $AZ_APP_NAME \
  --resource-group $AZ_RG \
  --location $AZ_LOCATION \
  --environment $AZ_CONTAINER_APP_ENV_NAME \
  --image 'dfberryregistry.azurecr.io/api-server/20241029201155' \
  --target-port 4000 \
  --ingress external \
  --query properties.configuration.ingress.fqdn