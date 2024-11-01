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

# SET FULL_IMAGE to $AZ_CONTAINER_REGISTRY_ENDPOINT/$IMAGE:20241029171748
FULL_IMAGE=$AZ_CONTAINER_REGISTRY_ENDPOINT/$IMAGE:20241029171748

# Deploy image
az containerapp up \
  --name $AZ_APP_NAME \
  --resource-group $AZ_RG \
  --location $AZ_LOCATION \
  --environment $AZ_CONTAINER_APP_ENV_NAME \
  --image '$FULL_IMAGE' \
  --target-port 4000 \
  --ingress external \
  --query properties.configuration.ingress.fqdn