#!/bin/bash
#
# MapToPoster Railway Deployment Script
#
# Usage:
#   ./scripts/deploy-railway.sh <RAILWAY_TOKEN> [BRANCH]
#
# Arguments:
#   RAILWAY_TOKEN  - Required. Your Railway API token
#   BRANCH         - Optional. Git branch to deploy (default: main)
#
# Examples:
#   ./scripts/deploy-railway.sh my-token-here
#   ./scripts/deploy-railway.sh my-token-here feature-branch
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
RAILWAY_API="https://backboard.railway.app/graphql/v2"
GITHUB_REPO="N3mes1s/maptoposter"
PROJECT_NAME="maptoposter"

# Parse arguments
TOKEN="${1:-}"
BRANCH="${2:-main}"

if [ -z "$TOKEN" ]; then
    echo -e "${RED}Error: Railway token is required${NC}"
    echo ""
    echo "Usage: $0 <RAILWAY_TOKEN> [BRANCH]"
    echo ""
    echo "Arguments:"
    echo "  RAILWAY_TOKEN  - Your Railway API token (required)"
    echo "  BRANCH         - Git branch to deploy (default: main)"
    exit 1
fi

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  MapToPoster Railway Deployment${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "Repository: ${GREEN}${GITHUB_REPO}${NC}"
echo -e "Branch:     ${GREEN}${BRANCH}${NC}"
echo ""

# Function to make GraphQL requests
graphql() {
    local query="$1"
    curl -sk -X POST "$RAILWAY_API" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $TOKEN" \
        -d "$query"
}

# Check if project exists
echo -e "${YELLOW}Checking for existing project...${NC}"
EXISTING_PROJECT=$(graphql '{"query": "query { projects { edges { node { id name } } } }"}' | \
    python3 -c "import sys, json; data = json.load(sys.stdin); projects = [p['node'] for p in data.get('data', {}).get('projects', {}).get('edges', [])]; match = next((p for p in projects if p['name'] == '$PROJECT_NAME'), None); print(match['id'] if match else '')" 2>/dev/null || echo "")

if [ -n "$EXISTING_PROJECT" ]; then
    echo -e "${GREEN}Found existing project: ${EXISTING_PROJECT}${NC}"
    PROJECT_ID="$EXISTING_PROJECT"
else
    echo -e "${YELLOW}Creating new project...${NC}"
    PROJECT_RESULT=$(graphql "{\"query\": \"mutation { projectCreate(input: { name: \\\"$PROJECT_NAME\\\" }) { id name } }\"}")
    PROJECT_ID=$(echo "$PROJECT_RESULT" | python3 -c "import sys, json; print(json.load(sys.stdin)['data']['projectCreate']['id'])" 2>/dev/null)

    if [ -z "$PROJECT_ID" ]; then
        echo -e "${RED}Failed to create project${NC}"
        echo "$PROJECT_RESULT"
        exit 1
    fi
    echo -e "${GREEN}Created project: ${PROJECT_ID}${NC}"
fi

# Get environment ID
echo -e "${YELLOW}Getting environment...${NC}"
ENV_RESULT=$(graphql "{\"query\": \"query { project(id: \\\"$PROJECT_ID\\\") { environments { edges { node { id name } } } } }\"}")
ENV_ID=$(echo "$ENV_RESULT" | python3 -c "import sys, json; envs = json.load(sys.stdin)['data']['project']['environments']['edges']; print(envs[0]['node']['id'] if envs else '')" 2>/dev/null)

if [ -z "$ENV_ID" ]; then
    echo -e "${RED}Failed to get environment${NC}"
    exit 1
fi
echo -e "${GREEN}Environment ID: ${ENV_ID}${NC}"

# Deploy from GitHub
echo -e "${YELLOW}Deploying from GitHub...${NC}"
DEPLOY_RESULT=$(graphql "{
    \"query\": \"mutation(\$input: GitHubRepoDeployInput!) { githubRepoDeploy(input: \$input) }\",
    \"variables\": {
        \"input\": {
            \"repo\": \"$GITHUB_REPO\",
            \"branch\": \"$BRANCH\",
            \"projectId\": \"$PROJECT_ID\",
            \"environmentId\": \"$ENV_ID\"
        }
    }
}")

SERVICE_ID=$(echo "$DEPLOY_RESULT" | python3 -c "import sys, json; print(json.load(sys.stdin)['data']['githubRepoDeploy'])" 2>/dev/null)

if [ -z "$SERVICE_ID" ]; then
    echo -e "${RED}Failed to deploy${NC}"
    echo "$DEPLOY_RESULT"
    exit 1
fi
echo -e "${GREEN}Service ID: ${SERVICE_ID}${NC}"

# Set environment variables
echo -e "${YELLOW}Configuring environment variables...${NC}"
graphql "{
    \"query\": \"mutation(\$input: VariableCollectionUpsertInput!) { variableCollectionUpsert(input: \$input) }\",
    \"variables\": {
        \"input\": {
            \"projectId\": \"$PROJECT_ID\",
            \"environmentId\": \"$ENV_ID\",
            \"serviceId\": \"$SERVICE_ID\",
            \"variables\": {
                \"API_HOST\": \"0.0.0.0\",
                \"CORS_ORIGINS\": \"*\",
                \"LOG_LEVEL\": \"INFO\",
                \"DEFAULT_THEME\": \"feature_based\",
                \"OUTPUT_DPI\": \"150\",
                \"NOMINATIM_TIMEOUT\": \"15.0\",
                \"OSM_TIMEOUT\": \"120.0\"
            }
        }
    }
}" > /dev/null
echo -e "${GREEN}Environment variables configured${NC}"

# Check for existing domain
echo -e "${YELLOW}Setting up domain...${NC}"
DOMAIN_CHECK=$(graphql "{\"query\": \"query { service(id: \\\"$SERVICE_ID\\\") { serviceDomains { domain } } }\"}")
EXISTING_DOMAIN=$(echo "$DOMAIN_CHECK" | python3 -c "import sys, json; domains = json.load(sys.stdin).get('data', {}).get('service', {}).get('serviceDomains', []); print(domains[0]['domain'] if domains else '')" 2>/dev/null || echo "")

if [ -n "$EXISTING_DOMAIN" ]; then
    DOMAIN="$EXISTING_DOMAIN"
    echo -e "${GREEN}Using existing domain: ${DOMAIN}${NC}"
else
    DOMAIN_RESULT=$(graphql "{
        \"query\": \"mutation { serviceDomainCreate(input: { serviceId: \\\"$SERVICE_ID\\\", environmentId: \\\"$ENV_ID\\\" }) { domain } }\"
    }")
    DOMAIN=$(echo "$DOMAIN_RESULT" | python3 -c "import sys, json; print(json.load(sys.stdin)['data']['serviceDomainCreate']['domain'])" 2>/dev/null)
    echo -e "${GREEN}Created domain: ${DOMAIN}${NC}"
fi

# Wait for deployment
echo ""
echo -e "${YELLOW}Waiting for deployment...${NC}"
MAX_ATTEMPTS=60
ATTEMPT=0

while [ $ATTEMPT -lt $MAX_ATTEMPTS ]; do
    DEPLOY_STATUS=$(graphql "{\"query\": \"query { service(id: \\\"$SERVICE_ID\\\") { deployments(first: 1) { edges { node { status } } } } }\"}")
    STATUS=$(echo "$DEPLOY_STATUS" | python3 -c "import sys, json; deps = json.load(sys.stdin)['data']['service']['deployments']['edges']; print(deps[0]['node']['status'] if deps else 'UNKNOWN')" 2>/dev/null)

    case "$STATUS" in
        "SUCCESS")
            echo -e "${GREEN}Deployment successful!${NC}"
            break
            ;;
        "FAILED"|"CRASHED")
            echo -e "${RED}Deployment failed with status: ${STATUS}${NC}"
            exit 1
            ;;
        "BUILDING"|"DEPLOYING"|"INITIALIZING"|"WAITING")
            echo -ne "\r${YELLOW}Status: ${STATUS}... (${ATTEMPT}/${MAX_ATTEMPTS})${NC}    "
            ;;
        *)
            echo -ne "\r${YELLOW}Status: ${STATUS}... (${ATTEMPT}/${MAX_ATTEMPTS})${NC}    "
            ;;
    esac

    sleep 5
    ATTEMPT=$((ATTEMPT + 1))
done

if [ $ATTEMPT -ge $MAX_ATTEMPTS ]; then
    echo -e "${YELLOW}Deployment still in progress. Check Railway dashboard.${NC}"
fi

echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}Deployment Complete!${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "Application URL: ${GREEN}https://${DOMAIN}${NC}"
echo -e "Health Check:    ${GREEN}https://${DOMAIN}/health${NC}"
echo -e "API Docs:        ${GREEN}https://${DOMAIN}/api/docs${NC}"
echo ""
echo -e "Railway Dashboard: ${BLUE}https://railway.app/project/${PROJECT_ID}${NC}"
echo ""
