################################################################################
#
# You can run them like `make install` or `make benchmark`
# 
################################################################################


################################################################################
# INSTALL
################################################################################

install-jq:
ifeq ($(shell lsb_release -si), ManjaroLinux) 
	sudo pacman -S jq --noconfirm
else ifeq ($(shell lsb_release -si), ArchLinux) 
	sudo pacman -S jq --noconfirm
else ifeq ($(shell uname),Linux)
	sudo apt-get install jq -y
else 
	brew install jq --quiet --force
endif

install-csv-to-markdown:
	sudo npm install --global csv-to-markdown

install mpdf:
	sudo npm install mpdf -g

install: install-jq
install: install-csv-to-markdown
install: install-mpdf


################################################################################
# BENCHMARK
################################################################################

benchmark-build:
ifeq ($(shell uname),Linux)
	cargo build \
		--profile release \
		--features runtime-benchmarks
else 
	AR=/usr/local/opt/llvm/bin/llvm-ar \
	CC=/usr/local/opt/llvm/bin/clang \
	cargo build \
	--release \
	--features runtime-benchmarks
endif

benchmark-compute:
	./target/release/node-template benchmark pallet \
              --chain dev \
              --execution=wasm \
              --wasm-execution=compiled \
              --pallet "*" \
              --extrinsic "*" \
              --steps 50 \
              --repeat 20 \
              --json \
              > results.json

benchmark-to-csv:
	cat results.json | jq -r '["extrinsic", "time (µs)"], (.[] | [ .benchmark, ([.time_results[]?.extrinsic_time] | (add / (if length == 0 then 1 else length end)) | round /1000 ) ]) | @csv' > results.csv

benchmark-csv-to-markdown:
	csv-to-markdown results.csv > benchmark.md

benchmark-to-pdf:
	mpdf benchmark.md

benchmark-cleanup:
	rm results.json
	rm results.csv
	rm benchmark.md


benchmark: benchmark-build
benchmark: benchmark-compute
benchmark: benchmark-to-csv
benchmark: benchmark-csv-to-markdown
benchmark: benchmark-to-pdf
benchmark: benchmark-cleanup

    

