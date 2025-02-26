DOMAIN ?= incheonheroes.world

node_modules:
	npm install

.PHONY: build help deploy
build: node_modules
	@npx hardhat compile

deploy:
	@echo "Deploying IncheonContents contract..."
	$(DEPLOY_ENVS) node cli.js deploy IncheonContents $(CLI_CONTRACT_OWNER_ADDR) "Incheon Heroes Contents" "IHC" 'https://$(DOMAIN)/metadata/json/{id}.json'

help:
	node cli.js -h

list:
	node cli.js list
