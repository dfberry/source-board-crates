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

# Create a tag from datetime
TAG=$(date +%Y%m%d%H%M%S)
echo "Tag: $TAG"

az containerapp update \
  --subscription $AZ_SUB_ID \
  --name $AZ_APP_NAME \
  --resource-group $AZ_RG \
  --set-env-vars PORT=secretref:PORT

  az containerapp update \
  --subscription $AZ_SUB_ID \
  --name $AZ_APP_NAME \
  --resource-group $AZ_RG \
  --set-env-vars DATABASE_URL=secretref:DATABASE_URL

# Build the image, send to registry
az acr build --image $IMAGE:$TAG \
  --registry $AZ_CONTAINER_REGISTRY_NAME \
  --file Dockerfile . 
echo "Image built and pushed to registry"

# Run the image
az acr run --registry $AZ_CONTAINER_REGISTRY_NAME \
  --cmd '$AZ_CONTAINER_REGISTRY_ENDPOINT/$IMAGE:$TAG' /dev/null
echo "Image running in registry"

# Deploy image
az containerapp up \
  --name $AZ_APP_NAME \
  --resource-group $AZ_RG \
  --location $AZ_LOCATION \
  --environment $AZ_CONTAINER_APP_ENV_NAME \
  --image '$AZ_CONTAINER_REGISTRY_ENDPOINT/$IMAGE:$TAG' \
  --target-port 4000 \
  --ingress external \
  --query properties.configuration.ingress.fqdn


  