PROJECT ?= incheonheroes
SERVICE ?= main-api
ENV ?= dev

WORKSPACE_ROOT ?= $(PWD)/../..
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
API_MODEL_ARTIFACT_DIR ?= $(PWD)/.build/api-models
CARGO_TARGET_DIR ?= $(PWD)/target
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)

BUCKET_NAME ?= dev.incheonheroes.world
BUCKET_EXPIRE ?= 3600
ASSET_DIR ?= metadata

KLAYTN_FEEPAYER_KEY ?= $(DEV_KEY)
KLAYTN_FEEPAYER_ADDR ?= $(DEV_ADDR)
KLAYTN_OWNER_KEY ?= $(DEV_KEY)
KLAYTN_OWNER_ADDR ?= $(DEV_ADDR)
KLAYTN_ENDPOINT ?= https://public-en-kairos.node.kaia.io
CONTRACT_INCHEON_CONTENTS ?= "0x54a01cd46b7f585584Fe4a3F7dafC25EE1fac0c9"
CONTRACT_ACCOUNT_PROFILE ?= "0x49ee26D4dc8d52309e30fa190eBCe33fB8BF6279"
JWT_SECRET_KEY ?= ""

RUST_LOG ?= debug

BUILD_ENV ?= ENV=$(ENV) RUST_LOG=$(RUST_LOG) VERSION=$(VERSION) COMMIT=$(COMMIT) DATABASE_TYPE=$(DATABASE_TYPE) DATABASE_URL=$(DATABASE_URL) API_MODEL_ARTIFACT_DIR=$(API_MODEL_ARTIFACT_DIR) CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) BUCKET_NAME=$(BUCKET_NAME) AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) AWS_REGION=$(AWS_REGION) ASSET_DIR=$(ASSET_DIR) BUCKET_EXPIRE=$(BUCKET_EXPIRE) KLAYTN_FEEPAYER_KEY=$(KLAYTN_FEEPAYER_KEY) KLAYTN_FEEPAYER_ADDR=$(KLAYTN_FEEPAYER_ADDR) KLAYTN_OWNER_KEY=$(KLAYTN_OWNER_KEY) KLAYTN_OWNER_ADDR=$(KLAYTN_OWNER_ADDR) KLAYTN_ENDPOINT=$(KLAYTN_ENDPOINT) CONTRACT_INCHEON_CONTENTS=$(CONTRACT_INCHEON_CONTENTS) JWT_SECRET_KEY=$(JWT_SECRET_KEY) CONTRACT_ACCOUNT_PROFILE=$(CONTRACT_ACCOUNT_PROFILE)

run:
	$(BUILD_ENV) cargo watch -x run -w src

build:
	$(BUILD_ENV) cargo build --release -p $(SERVICE) --features lambda
	mkdir -p $(WORKSPACE_ROOT)/.build/$(SERVICE)
	cp $(CARGO_TARGET_DIR)/release/$(SERVICE) $(WORKSPACE_ROOT)/.build/$(SERVICE)/bootstrap
