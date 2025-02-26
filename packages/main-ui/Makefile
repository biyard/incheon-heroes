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
MAIN_API_ENDPOINT ?= https://api.incheon.world
DISCORD_API_ENDPOINT ?= https://discord-api.incheon.world
LOGS_API_ENDPOINT ?= https://logs-api.incheon.world
NFT_BASE_URI ?= https://metadata.biyard.co/incheon-heroes/json
WORKSPACE_ROOT ?= ../..

ICP_CANISTER_ID ?= 5wtzb-piaaa-aaaap-qhjfq-cai
ICP_ENDPOINT ?= https://identity.ic0.app

KLAYTN_ENDPOINT ?= https://public-en.node.kaia.io/
CONTRACT_SHOP ?= "0x3f51e2216F6E4532324e70d71073696Ef2A73b7e"
CONTRACT_HOLDER ?= "0x03AcEF98A25B8bB8fa47F0dbF0116823E6f65528"
CONTRACT_SBT ?= "0xFa9d777647BF54A6052E21FdC6A3ecd60CF0E076"
CONTRACT_EXPERIENCE ?= "0x7cD4A0339Bb7DAF2088E02d6E3933fDBc9e9F360"
CONTRACT_NFT ?= "0xa3bd085014a9Ea59CBE9E14e6A9220E83BbCA333"
CONTRACT_MISSION ?= "0xC95261343FCDdbb8c0885FE2E095a032DE486728"

BUILD_ENV ?= ENV=$(ENV) VERSION=$(VERSION) COMMIT=$(COMMIT) RUST_LOG=$(RUST_LOG) ENV=$(ENV) DOMAIN=${DOMAIN} AWS_REGION=${REGION} AWS_ACCESS_KEY_ID=${ACCESS_KEY_ID} AWS_SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY} SERVICE=$(SERVICE) WORKSPACE_ROOT=$(WORKSPACE_ROOT) BASE_URL=$(BASE_URL) MAIN_API_ENDPOINT=$(MAIN_API_ENDPOINT) DISCORD_API_ENDPOINT=$(DISCORD_API_ENDPOINT) LOGS_API_ENDPOINT=$(LOGS_API_ENDPOINT) REDIRECT_URI=$(REDIRECT_URI) NFT_BASE_URI=$(NFT_BASE_URI) CONTRACT_SHOP=$(CONTRACT_SHOP) KLAYTN_ENDPOINT=$(KLAYTN_ENDPOINT) CONTRACT_HOLDER=$(CONTRACT_HOLDER) CONTRACT_EXPERIENCE=$(CONTRACT_EXPERIENCE) CONTRACT_NFT=$(CONTRACT_NFT) CONTRACT_SBT=$(CONTRACT_SBT) CONTRACT_MISSION=$(CONTRACT_MISSION) ICP_CANISTER_ID=$(ICP_CANISTER_ID) ICP_ENDPOINT=$(ICP_ENDPOINT)

ifeq ("$(ENV)","local")
	MODE = development
	FLAGS =
else
	MODE = production
endif

setup.tool:
	cargo install dioxus-cli --version 0.6.3
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

.PHONY: public/tailwind.css
public/tailwind.css:
	npx tailwindcss -i ./public/input.css -o ./public/tailwind.css --minify

clean:
	rm -rf public/tailwind.css public/dep.js

build-docker: clean public/tailwind.css
	docker run -it --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) biyard/dioxus-docker bash -c 'source ~/.cargo/env && $(BUILD_ENV) dx build --release --fullstack --server-features lambda && cp -r /app/target/dx/$(SERVICE)/release/web /app/.build/$(SERVICE) && mv /app/.build/$(SERVICE)/server /app/.build/$(SERVICE)/bootstrap'
