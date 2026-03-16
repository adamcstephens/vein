#!/usr/bin/env bash

ENVRC="../../.secret.envrc"

cd "$(echo "$DIRENV_DIR/.services/vikunja" | sed 's/^-//')" || exit 1

if vikunja user list 2>&1 | grep -q 'admin'; then
  echo "Admin user already exists, skipping."
else
  vikunja user create -u admin -e admin@dev.local -p admin
  echo "Admin user created."
fi

# Get a JWT for API calls
JWT=$(curl -sf localhost:3456/api/v1/login \
  -d '{"username":"admin1","password":"admin"}' \
  -H 'Content-Type: application/json' | gojq -r '.token')

if grep -q 'VIKUNJA_API_TOKEN' "$ENVRC" 2>/dev/null; then
  echo "API token already in .secret.envrc, skipping."
else
  TOKEN=$(curl -sf -X PUT localhost:3456/api/v1/tokens \
    -H "Authorization: Bearer $JWT" \
    -H 'Content-Type: application/json' \
    -d '{"title":"dev-token","permissions":{"projects":["read_all","create","update","delete"],"tasks":["read_all","read_one","create","update","delete"],"labels":["read_all","create"],"tasks_labels":["create"],"projects_views_buckets_tasks":["update"]},"expires_at":"2030-01-01T00:00:00Z"}' |
    gojq -r '.token')
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
  BUCKETS=$(curl -sf "localhost:3456/api/v1/projects/$PROJECT_ID/views/$VIEW_ID/buckets" \
    -H "Authorization: Bearer $JWT")
  TODO_BUCKET_ID=$(echo "$BUCKETS" | gojq -r '.[0].id')
  INPROGRESS_BUCKET_ID=$(echo "$BUCKETS" | gojq -r '.[1].id')
  DONE_BUCKET_ID=$(echo "$BUCKETS" | gojq -r '.[2].id')
  echo "export VIKUNJA_PROJECT_ID=\"$PROJECT_ID\"" >>"$ENVRC"
  echo "export VIKUNJA_VIEW_ID=\"$VIEW_ID\"" >>"$ENVRC"
  echo "export VIKUNJA_TODO_BUCKET_ID=\"$TODO_BUCKET_ID\"" >>"$ENVRC"
  echo "export VIKUNJA_INPROGRESS_BUCKET_ID=\"$INPROGRESS_BUCKET_ID\"" >>"$ENVRC"
  echo "export VIKUNJA_DONE_BUCKET_ID=\"$DONE_BUCKET_ID\"" >>"$ENVRC"
  echo "Dev project created (project=$PROJECT_ID, view=$VIEW_ID, buckets=$TODO_BUCKET_ID/$INPROGRESS_BUCKET_ID/$DONE_BUCKET_ID)"
fi

echo "Done — run: direnv allow"
