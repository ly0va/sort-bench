all : cpp/merge go/merge rust/merge

rust/merge: rust/src/main.rs
	cargo build --release --manifest-path=rust/Cargo.toml
	ln -srf rust/target/release/merge rust/merge

cpp/merge : cpp/merge.cpp
	g++ -o $@ -O3 $< -pthread -fopenmp

go/merge : go/merge.go
	go build -o $@ $<

clean :
	rm -rf cpp/merge go/merge rust/{target,merge}

.PHONY : clean all


