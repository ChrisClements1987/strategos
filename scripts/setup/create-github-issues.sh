#!/bin/bash
# Script to create GitHub issues from v0.1 backlog
# Run this from the project root: bash scripts/setup/create-github-issues.sh

# v0.1 Issues - Core Engine (E-1.1)
gh issue create --title "Portfolio Management (CRUD)" \
  --body "Implement full CRUD operations for Portfolio entity. Includes database schema, Rust backend API, and frontend integration." \
  --label "v0.1,theme:core-engine,epic:E-1.1,type:feature"

gh issue create --title "Product Management (CRUD)" \
  --body "Implement full CRUD operations for Product entity. Includes database schema, Rust backend API, and frontend integration." \
  --label "v0.1,theme:core-engine,epic:E-1.1,type:feature"

gh issue create --title "Feature Management (CRUD)" \
  --body "Implement full CRUD operations for Feature entity. Includes database schema, Rust backend API, and frontend integration." \
  --label "v0.1,theme:core-engine,epic:E-1.1,type:feature"

gh issue create --title "Licence Management (CRUD)" \
  --body "Implement full CRUD operations for Licence entity. Includes database schema, Rust backend API, and frontend integration." \
  --label "v0.1,theme:core-engine,epic:E-1.1,type:feature"

gh issue create --title "Client/Persona Management (CRUD)" \
  --body "Implement full CRUD operations for Client/Persona entity. Includes database schema, Rust backend API, and frontend integration." \
  --label "v0.1,theme:core-engine,epic:E-1.1,type:feature"

gh issue create --title "Requirement Management (CRUD)" \
  --body "Implement full CRUD operations for Requirement entity. Includes database schema, Rust backend API, and frontend integration." \
  --label "v0.1,theme:core-engine,epic:E-1.1,type:feature"

# v0.1 Issues - Dynamic Views (E-2.1)
gh issue create --title "Dynamic Phase (Roadmap) View" \
  --body "Create a dynamic view for displaying roadmap phases. Should allow filtering, sorting, and visualization of timeline data." \
  --label "v0.1,theme:dynamic-views,epic:E-2.1,type:feature"

gh issue create --title "Dynamic Product View" \
  --body "Create a dynamic view for displaying products. Should show product hierarchy, modules, and associated features." \
  --label "v0.1,theme:dynamic-views,epic:E-2.1,type:feature"

gh issue create --title "Dynamic Licence View" \
  --body "Create a dynamic view for displaying licences and their relationships to products and features." \
  --label "v0.1,theme:dynamic-views,epic:E-2.1,type:feature"

gh issue create --title "Dynamic Client View" \
  --body "Create a dynamic view for displaying clients/personas and their associated requirements and licences." \
  --label "v0.1,theme:dynamic-views,epic:E-2.1,type:feature"

# v0.1 Issues - App Shell (E-5.1, E-5.2)
gh issue create --title "Tauri Cross-Platform Build" \
  --body "Set up and test cross-platform builds for Windows, macOS, and Linux. Configure GitHub Actions for automated builds." \
  --label "v0.1,theme:app-shell,epic:E-5.1,type:feature"

gh issue create --title "Documentation (README, User Guides)" \
  --body "Create comprehensive documentation including README, user guides, developer documentation, and API docs." \
  --label "v0.1,theme:app-shell,epic:E-5.2,type:docs"

echo "✅ All v0.1 issues created successfully!"
