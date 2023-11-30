.PHONY: all
all:
	make pack

.PHONY: pack
pack:
	substreams pack substreams.mainnet.yaml
	substreams pack substreams.polygon.yaml
	substreams pack substreams.bnb.yaml
