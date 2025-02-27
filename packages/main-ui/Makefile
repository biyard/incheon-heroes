SERVICE ?= main-ui
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)

RUST_LOG ?= debug
REDIRECT_URI ?= http://localhost:3000
WORKSPACE_ROOT ?= ../..
MAIN_API_ENDPOINT ?= https://dev-api.incheon.world
DISCORD_API_ENDPOINT ?= https://dev-discord-api.incheon.world
LOGS_API_ENDPOINT ?= https://dev-logs-api.incheon.world
NFT_BASE_URI ?= https://metadata.biyard.co/incheon-heroes/json
WORKSPACE_ROOT ?= ../..

ICP_CANISTER_ID ?= mkam6-f4aaa-aaaaa-qablq-cai
ICP_ENDPOINT ?= https://rdmx6-jaaaa-aaaaa-aaadq-cai.icp.biyard.co

KLAYTN_ENDPOINT ?= https://public-en-kairos.node.kaia.io
CONTRACT_SHOP ?= "0xd1C122aBCa35B5187CccF98FEb1ddF46E3db5367"
CONTRACT_HOLDER ?= "0xCeE4aAaD519D03D92C31402ea4dA5A0e7CA6b9D4"
CONTRACT_SBT ?= "0xA7655552f1E170A2B2C4193E0a567908fb3a38B9"
CONTRACT_EXPERIENCE ?= "0x24e9B346D95Af3220A1022861a1A3ce936B5D6C2"
CONTRACT_NFT ?= "0x7F3A142E8bbb68098159ff5Ff6557c6C99325B8b"
CONTRACT_MISSION ?= "0x70ADA463d16F52ade7C3D03E1864c7B89061B01f"
KLAYTN_SCOPE_ENDPOINT ?= https://kairos.kaiascope.com/tx
NEW__API_ENDPOINT ?= https://api.dev.incheonheroes.world

CONTRACT_INCHEON_CONTENTS ?= "0x54a01cd46b7f585584Fe4a3F7dafC25EE1fac0c9"
OPENSEA_ENDPOINT ?= "https://testnets.opensea.io/assets/baobab"

DISCORD_MISSION_URL ?= "https://discord.com/channels/1140593363515617372/1169178052647985213"

BUILD_ENV ?= ENV=$(ENV) VERSION=$(VERSION) COMMIT=$(COMMIT) RUST_LOG=$(RUST_LOG) ENV=$(ENV) DOMAIN=${DOMAIN} AWS_REGION=${REGION} AWS_ACCESS_KEY_ID=${ACCESS_KEY_ID} AWS_SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY} SERVICE=$(SERVICE) WORKSPACE_ROOT=$(WORKSPACE_ROOT) BASE_URL=$(BASE_URL) MAIN_API_ENDPOINT=$(MAIN_API_ENDPOINT) DISCORD_API_ENDPOINT=$(DISCORD_API_ENDPOINT) LOGS_API_ENDPOINT=$(LOGS_API_ENDPOINT) REDIRECT_URI=$(REDIRECT_URI) NFT_BASE_URI=$(NFT_BASE_URI) CONTRACT_SHOP=$(CONTRACT_SHOP) KLAYTN_ENDPOINT=$(KLAYTN_ENDPOINT) CONTRACT_HOLDER=$(CONTRACT_HOLDER) CONTRACT_EXPERIENCE=$(CONTRACT_EXPERIENCE) CONTRACT_NFT=$(CONTRACT_NFT) CONTRACT_SBT=$(CONTRACT_SBT) CONTRACT_MISSION=$(CONTRACT_MISSION) ICP_CANISTER_ID=$(ICP_CANISTER_ID) ICP_ENDPOINT=$(ICP_ENDPOINT) KLAYTN_SCOPE_ENDPOINT=$(KLAYTN_SCOPE_ENDPOINT) NEW_API_ENDPOINT=$(NEW_API_ENDPOINT) FEEPAYER_ADDRESS=$(FEEPAYER_ADDRESS) OPENSEA_ENDPOINT=$(OPENSEA_ENDPOINT) CONTRACT_INCHEON_CONTENTS=$(CONTRACT_INCHEON_CONTENTS) DISCORD_MISSION_URL=$(DISCORD_MISSION_URL)

ifeq ("$(ENV)","local")
	MODE = development
	FLAGS =
else
	MODE = production
endif

setup.tool:
	cargo install dioxus-cli --version 0.6.0
	cargo install toml-cli
	npm i -g tailwindcss

serve: clean public/tailwind.css
	$(BUILD_ENV) dx serve --addr 0.0.0.0 --platform web  --client-features web-only

run: clean node_modules public
	$(BUILD_ENV) dx serve --fullstack

node_modules:
	npm i

build: clean node_modules public
	$(BUILD_ENV) dx build --release --fullstack --server-features lambda
	cp -r $(WORKSPACE_ROOT)/target/dx/$(SERVICE)/release/web $(ARTIFACT_DIR)
	mv $(ARTIFACT_DIR)/server $(ARTIFACT_DIR)/bootstrap

.PHONY: public
public: public/tailwind.css

.PHONY: public/dep.js
public/dep.js:
	cd js && npx webpack --mode $(MODE)
	cp js/dist/dep.js public/dep.js

.PHONY: public/dep.js
public/tailwind.css:
	tailwindcss -i ./public/input.css -o ./public/tailwind.css --minify

clean:
	rm -rf public/tailwind.css public/dep.js

build-docker: clean public/tailwind.css
	docker run -it --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) biyard/dioxus-docker bash -c 'source ~/.cargo/env && $(BUILD_ENV) dx build --release --fullstack --server-features lambda && cp -r /app/target/dx/$(SERVICE)/release/web /app/.build/$(SERVICE) && mv /app/.build/$(SERVICE)/server /app/.build/$(SERVICE)/bootstrap'
