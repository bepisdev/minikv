CARGO := $(shell which cargo)
OUTDIR := dist
TARGETDIR := target
BIN := minikv

debug:
	@mkdir -p $(OUTDIR)
	$(CARGO) build \
		--verbose
	cp $(TARGETDIR)/debug/$(BIN) $(OUTDIR)/$(BIN)

release:
	@mkdir -p $(OUTDIR)
	$(CARGO) build \
		--verbose \
		--release
	cp $(TARGETDIR)/release/$(BIN) $(OUTDIR)/$(BIN)

test:
	$(CARGO) test


.PHONY: debug release test
