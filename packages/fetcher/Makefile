PROJECT ?= incheonheroes
SERVICE ?= fetcher
ENV ?= dev
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)

WORKSPACE_ROOT ?= $(PWD)/../..
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
API_MODEL_ARTIFACT_DIR ?= $(PWD)/.build/api-models
CARGO_TARGET_DIR ?= $(PWD)/target

KLAYTN_FEEPAYER_KEY ?= $(DEV_KEY)
KLAYTN_FEEPAYER_ADDR ?= $(DEV_ADDR)
KLAYTN_OWNER_KEY ?= $(DEV_KEY)
KLAYTN_OWNER_ADDR ?= $(DEV_ADDR)
KLAYTN_ENDPOINT ?= https://public-en-kairos.node.kaia.io
CONTRACT_INCHEON_CONTENTS ?= 0x54a01cd46b7f585584Fe4a3F7dafC25EE1fac0c9

RUST_LOG ?= debug

BUILD_ENV ?= ENV=$(ENV) RUST_LOG=$(RUST_LOG) VERSION=$(VERSION) COMMIT=$(COMMIT) DATABASE_TYPE=$(DATABASE_TYPE) DATABASE_URL=$(DATABASE_URL) API_MODEL_ARTIFACT_DIR=$(API_MODEL_ARTIFACT_DIR) CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) BUCKET_NAME=$(BUCKET_NAME) AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) AWS_REGION=$(AWS_REGION) ASSET_DIR=$(ASSET_DIR) BUCKET_EXPIRE=$(BUCKET_EXPIRE) KLAYTN_FEEPAYER_KEY=$(KLAYTN_FEEPAYER_KEY) KLAYTN_FEEPAYER_ADDR=$(KLAYTN_FEEPAYER_ADDR) KLAYTN_OWNER_KEY=$(KLAYTN_OWNER_KEY) KLAYTN_OWNER_ADDR=$(KLAYTN_OWNER_ADDR) KLAYTN_ENDPOINT=$(KLAYTN_ENDPOINT) CONTRACT_INCHEON_CONTENTS=$(CONTRACT_INCHEON_CONTENTS) JWT_SECRET_KEY=$(JWT_SECRET_KEY)

run:
	$(BUILD_ENV) cargo watch -x run -w src

build: bin.build docker.login docker.build docker.push

bin.build:
	$(BUILD_ENV) cargo build --release -p fetcher

docker.build:
	docker build -t $(REPO_NAME):$(COMMIT) -f Dockerfile $(WORKSPACE_ROOT)/target/release

docker.push:
	docker push $(REPO_NAME):$(COMMIT)

docker.login:
	aws ecr get-login-password --region $(REGION) | docker login --username AWS --password-stdin $(AWS_ACCOUNT_ID).dkr.ecr.$(REGION).amazonaws.com