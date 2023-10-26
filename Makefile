.PHONY: all
all:
	make build
	make pack
	make graph
	make info


.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: build
build: protogen
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack: build
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run: build
	substreams run substreams.yaml graph_out -e eth.substreams.pinax.network:9000 -s 1000000

.PHONY: gui
gui: build
	substreams gui substreams.yaml graph_out -e eth.substreams.pinax.network:9000 -s 1000000

