SERVICE ?= main-ui
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)

LOG_LEVEL ?= debug
REDIRECT_URI ?= http://localhost:3000
WORKSPACE_ROOT ?= ../..
MAIN_API_ENDPOINT ?= https://api.incheon.world
NFT_BASE_URI ?= https://metadata.biyard.co/incheon-heroes/json

BUILD_ENV ?= ENV=$(ENV) VERSION=$(VERSION) COMMIT=$(COMMIT) LOG_LEVEL=$(LOG_LEVEL) ENV=$(ENV) DOMAIN=${DOMAIN} AWS_REGION=${REGION} AWS_ACCESS_KEY_ID=${ACCESS_KEY_ID} AWS_SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY} SERVICE=$(SERVICE) WORKSPACE_ROOT=$(WORKSPACE_ROOT) BASE_URL=$(BASE_URL) MAIN_API_ENDPOINT=$(MAIN_API_ENDPOINT) REDIRECT_URI=$(REDIRECT_URI) NFT_BASE_URI=$(NFT_BASE_URI)

setup.tool:
	cargo install dioxus-cli --version 0.6.0
	cargo install toml-cli
	npm i -g tailwindcss

serve: clean public/tailwind.css
	$(BUILD_ENV) dx serve --addr 0.0.0.0 --platform web  --client-features web-only

run: clean public/tailwind.css
	$(BUILD_ENV) dx serve --fullstack

build: public/tailwind.css
	$(BUILD_ENV) dx build --release --fullstack --server-features lambda
	cp -r $(WORKSPACE_ROOT)/target/dx/$(SERVICE)/release/web $(ARTIFACT_DIR)
	mv $(ARTIFACT_DIR)/server $(ARTIFACT_DIR)/bootstrap

public/tailwind.css:
	tailwindcss -i ./public/input.css -o ./public/tailwind.css --minify

clean:
	rm -rf public/tailwind.css

build-docker: clean public/tailwind.css
	docker run -it --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) biyard/dioxus-docker bash -c 'source ~/.cargo/env && $(BUILD_ENV) dx build --release --fullstack --server-features lambda && cp -r /app/target/dx/$(SERVICE)/release/web /app/.build/$(SERVICE) && mv /app/.build/$(SERVICE)/server /app/.build/$(SERVICE)/bootstrap'
