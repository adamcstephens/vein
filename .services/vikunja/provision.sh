#!/usr/bin/env bash
set -euo pipefail

ENVRC="../../.secret.envrc"

cd "$(echo "$DIRENV_DIR/.services/vikunja" | sed 's/^-//')" || exit 1

if vikunja user list 2>&1 | grep -q 'admin'; then
  echo "Admin user already exists, skipping."
else
  vikunja user create -u admin -e admin@dev.local -p admin
  echo "Admin user created."
  # New user means old tokens/projects are stale — start fresh
  truncate -s 0 "$ENVRC"
  echo "Cleared stale .secret.envrc"
fi

# Get a JWT for API calls
JWT=$(curl -sf localhost:3456/api/v1/login \
  -d '{"username":"admin","password":"admin"}' \
  -H 'Content-Type: application/json' | gojq -r '.token')

if [ -z "$JWT" ]; then
  echo "ERROR: Failed to get JWT — is Vikunja running on localhost:3456?" >&2
  exit 1
fi

if grep -q 'VIKUNJA_API_TOKEN' "$ENVRC" 2>/dev/null; then
  echo "API token already in .secret.envrc, skipping."
else
  TOKEN=$(curl -sf -X PUT localhost:3456/api/v1/tokens \
    -H "Authorization: Bearer $JWT" \
    -H 'Content-Type: application/json' \
    -d '{"title":"dev-token","permissions":{"projects":["read_all","read_one","create","update","delete","views_buckets","views_buckets_tasks"],"tasks":["read_all","read_one","create","update","delete"],"labels":["read_all","create"],"tasks_labels":["create"],"projects_views_tasks":["read_all"]},"expires_at":"2030-01-01T00:00:00Z"}' |
    gojq -r '.token')
  if [ -z "$TOKEN" ]; then
    echo "ERROR: Failed to create API token" >&2
    exit 1
  fi
  echo "export VIKUNJA_API_TOKEN=\"$TOKEN\"" >>"$ENVRC"
  echo "API token written to .secret.envrc"
fi

if grep -q 'VIKUNJA_PROJECT_ID' "$ENVRC" 2>/dev/null; then
  echo "Dev project already in .secret.envrc, skipping."
else
  PROJECT=$(curl -sf -X PUT localhost:3456/api/v1/projects \
    -H "Authorization: Bearer $JWT" \
    -H 'Content-Type: application/json' \
    -d '{"title":"vein-dev"}')
  PROJECT_ID=$(echo "$PROJECT" | gojq -r '.id')
  VIEW_ID=$(echo "$PROJECT" | gojq -r '[.views[] | select(.view_kind == "kanban")][0].id')
  if [ -z "$PROJECT_ID" ] || [ "$PROJECT_ID" = "null" ]; then
    echo "ERROR: Failed to create project" >&2
    exit 1
  fi
  if [ -z "$VIEW_ID" ] || [ "$VIEW_ID" = "null" ]; then
    echo "ERROR: No kanban view found on project" >&2
    exit 1
  fi
  BUCKETS=$(curl -sf "localhost:3456/api/v1/projects/$PROJECT_ID/views/$VIEW_ID/buckets" \
    -H "Authorization: Bearer $JWT")
  TODO_BUCKET_ID=$(echo "$BUCKETS" | gojq -r '.[0].id')
  INPROGRESS_BUCKET_ID=$(echo "$BUCKETS" | gojq -r '.[1].id')
  DONE_BUCKET_ID=$(echo "$BUCKETS" | gojq -r '.[2].id')
  if [ -z "$TODO_BUCKET_ID" ] || [ "$TODO_BUCKET_ID" = "null" ]; then
    echo "ERROR: Failed to fetch buckets — expected at least 3 buckets on the kanban view" >&2
    exit 1
  fi
  echo "export VIKUNJA_PROJECT_ID=\"$PROJECT_ID\"" >>"$ENVRC"
  echo "export VIKUNJA_VIEW_ID=\"$VIEW_ID\"" >>"$ENVRC"
  echo "export VIKUNJA_TODO_BUCKET_ID=\"$TODO_BUCKET_ID\"" >>"$ENVRC"
  echo "export VIKUNJA_INPROGRESS_BUCKET_ID=\"$INPROGRESS_BUCKET_ID\"" >>"$ENVRC"
  echo "export VIKUNJA_DONE_BUCKET_ID=\"$DONE_BUCKET_ID\"" >>"$ENVRC"
  echo "Dev project created (project=$PROJECT_ID, view=$VIEW_ID, buckets=$TODO_BUCKET_ID/$INPROGRESS_BUCKET_ID/$DONE_BUCKET_ID)"
fi

echo "Done — run: direnv allow"
