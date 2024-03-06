ENDPOINT ?= mainnet.eth.streamingfast.io:443
# //this one might not be an erc20
# START_BLOCK ?= 14500327 
#nft no data
START_BLOCK ?= 16209658

# START_BLOCK ?= 14496745 
# START_BLOCK ?= 14336882
# //this gets everything but name
# START_BLOCK ?= 15347256
OTHER_BLOCK ?= 14159918
PUDGY ?= 12876179
STOP_BLOCK ?= +1

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml map_deployments -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_erc721_test -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="google,sf/substreams/rpc,sf/substreams/v1,sf/substreams/sink"

.PHONY: pack
pack: build
	substreams pack substreams.yaml
