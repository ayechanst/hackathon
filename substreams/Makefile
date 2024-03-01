ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 12876479
OTHER_BLOCK ?= 14159918
PUDGY ?= 12876179
STOP_BLOCK ?= +3

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml map_erc721_test -s $(PUDGY) -t $(STOP_BLOCK)

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_erc721_test -s $(PUDGY) -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="google,sf/substreams/rpc,sf/substreams/v1,sf/substreams/sink"

.PHONY: pack
pack: build
	substreams pack substreams.yaml
