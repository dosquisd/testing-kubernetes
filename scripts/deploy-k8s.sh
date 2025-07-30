#!/bin/bash

# Function to display usage
usage() {
    echo "Usage: $0 <environment> <resource-type>"
    echo "Available environments:"
    echo "  development   - Deploy to development environment"
    echo "  staging       - Deploy to staging environment"
    echo "  production    - Deploy to production environment"
    echo ""
    echo "Available resource types:"
    echo "  deployments   - Deploy all deployment files"
    echo "  configmaps    - Deploy all configmap files"
    echo "  secrets       - Deploy all secret files"
    echo "  services      - Deploy all service files"
    echo "  all           - Deploy all resources"
    exit 1
}

# Function to deploy specific resource type
deploy_resource() {
    local environment=$1
    local resource_type=$2
    echo "Deploying ${resource_type} to ${environment}..."
    
    find k8s/${environment}/ \
        -name "*-${resource_type}.yaml" \
        -o -name "*-${resource_type}.yml" | \
        while read -r file; do
            kubectl apply -f "$file"
        done
}

# Function to deploy secrets with environment substitution
deploy_secrets() {
    local environment=$1
    echo "Deploying secrets to ${environment}..."
    
    # Source and export all variables from .secrets.env
    if [ -f ".secrets.env" ]; then
        set -a
        source .secrets.env
        set +a
    fi
    
    find k8s/${environment}/ \
        -name "*-secrets.yaml" \
        -o -name "*-secrets.yml" | \
        while read -r file; do
            envsubst < "$file" | kubectl apply -f -
        done
}

# Check if both parameters are provided
if [ $# -lt 2 ]; then
    usage
fi

environment=$1
resource_type=$2

# Validate environment
case "$environment" in
    "development"|"staging"|"production")
        # Valid environment
        ;;
    *)
        echo "Error: Unknown environment '$environment'"
        usage
        ;;
esac

# Check if environment directory exists
if [ ! -d "k8s/${environment}" ]; then
    echo "Error: Directory k8s/${environment} does not exist"
    exit 1
fi

# Main logic for resource types
case "$resource_type" in
    "deployments")
        deploy_resource "$environment" "deployment"
        ;;
    "configmaps")
        deploy_resource "$environment" "configmap"
        ;;
    "secrets")
        deploy_secrets "$environment"
        ;;
    "services")
        deploy_resource "$environment" "service"
        ;;
    "all")
        deploy_secrets "$environment"
        deploy_resource "$environment" "configmap"
        deploy_resource "$environment" "service"
        deploy_resource "$environment" "deployment"
        ;;
    *)
        echo "Error: Unknown resource type '$resource_type'"
        usage
        ;;
esac

echo "Deployment to ${environment} completed!"
