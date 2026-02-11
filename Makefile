#!make

# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
#                                                                             #
#      ____                               _                                   #
#     / __ \____  ___  ____  ____ _____  (_) ®                                #
#    / / / / __ \/ _ \/ __ \/ __ `/ __ \/ /                                   #
#   / /_/ / /_/ /  __/ / / / /_/ / /_/ / /                                    #
#   \____/ .___/\___/_/ /_/\__,_/ .___/_/                                     #
#       /_/                    /_/                                            #
#                                                                             #
#   The Largest Certified API Marketplace                                     #
#   Accelerate Digital Transformation • Simplify Processes • Lead Industry    #
#                                                                             #
#   ═══════════════════════════════════════════════════════════════════════   #
#                                                                             #
#   Project:        repokit                                                   #
#   Version:        0.1.0                                                     #
#   Author:         Francesco Bianco (@francescobianco)                       #
#   Copyright:      (c) 2025 Openapi®. All rights reserved.                   #
#   License:        MIT                                                       #
#   Maintainer:     Francesco Bianco                                          #
#   Contact:        https://openapi.com/                                      #
#   Repository:     https://github.com/openapi/openapi-cli                    #
#   Documentation:  https://console.openapi.com/                              #
#                                                                             #
#   ═══════════════════════════════════════════════════════════════════════   #
#                                                                             #
#   "Truth lies at the source of the stream."                                 #
#                                  — English Proverb                          #
#                                                                             #
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

## ==============
## Dev Operations
## ==============

dev-push:
	@git config credential.helper 'cache --timeout=3600'
	@git add .
	@git commit -am "Updated at $$(date)" || true
	@git push

dev-version:
	@grep '^version' Cargo.toml | head -n1 | cut -d '"' -f2

## ==================
## OAS Specifications
## ==================

oas-download:
	@echo "Downloading OpenAPI specs into oas/ ..."
	@errors=0; \
	while IFS= read -r url; do \
		[ -z "$$url" ] && continue; \
		filename=$$(basename "$$url"); \
		if curl -sSf -o "oas/$$filename" "$$url"; then \
			echo "  OK  $$filename"; \
		else \
			echo "  WARNING: failed to download $$url — check the URL in oas/urls.txt" >&2; \
			errors=$$((errors + 1)); \
		fi; \
	done < oas/00-list.txt; \
	echo ""; \
	if [ $$errors -gt 0 ]; then \
		echo "Done with $$errors warning(s). Fix the URLs above in oas/urls.txt and re-run."; \
	else \
		echo "All specs downloaded successfully."; \
	fi

## ===========
## Build & Run
## ===========

build:
	@cargo build

release:
	@cargo build --release

install:
	@cargo install --path .

## ==================
## Packaging Commands
## ==================

publish:
	@git add .
	@git commit -m "Update release $$(make -s dev-version)" || true
	@git push
	@cargo login
	@cargo publish

## =======
## Testing
## =======

test:
	@cargo test

test-commands:
	@bash tests/commands-test.sh
