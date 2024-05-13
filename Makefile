.PHONY: all
all:
	make pack

.PHONY: pack
pack:
	substreams pack substreams.yaml
