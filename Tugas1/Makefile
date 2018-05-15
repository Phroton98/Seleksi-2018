CARGO = cargo

CARGO_RUN_OPT = RUST_BACKTRACE=1

all: 
	@echo 'Cleaning the data..'
	@cd src && $(CARGO) clean 
	@echo 'Building the data'
	@cd src && $(CARGO) build 
	@echo 'Running the data' 
	@cd src && $(CARGO) run

clean: 
	# remove data and binary folder
	@echo 'Cleaning the data..'
	@cd src && $(CARGO) clean
	@rm -f data/*

build: 
	@# compile to binary (if you use interpreter, then do not implement it)
	@echo 'Building the data'
	@cd src && $(CARGO) build

run: # run your binary
	@echo 'Running the data' 
	@cd src && $(CARGO_RUN_OPT) $(CARGO) run