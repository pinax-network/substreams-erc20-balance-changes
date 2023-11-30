.PHONY: all
all:
	make pack

.PHONY: pack
pack:
	substreams pack substreams.yaml
	substreams pack substreams.polygon.yaml
	substreams pack substreams.bnb.yaml
	substreams pack substreams.mainnet.yaml
