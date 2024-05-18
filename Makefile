CARGO := $(shell which cargo)
OUTDIR := dist
TARGETDIR := target
BIN := minikv
PREFIX := /usr

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

clean:
	@rm -rf $(TARGETDIR)
	@rm -rf $(OUTDIR)

test:
	$(CARGO) test

install:
	cp $(OUTDIR)/$(BIN) $(PREFIX)/bin/$(BIN)


.PHONY: debug release test clean
