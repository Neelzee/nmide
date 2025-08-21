module-check:
	@( \
		tmpfile=$$(mktemp); \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/Cargo.toml" ]; then \
				printf "Checking $$p... "; \
				if (cd "$$p" && cargo c >/dev/null 2>&1); then \
					echo "✓"; \
				else \
					echo "✗"; \
					echo "$$p" >> "$$tmpfile"; \
				fi; \
			fi; \
		done; \
		if [ -s "$$tmpfile" ]; then \
			echo ""; \
			echo "❌ Failed crates:"; \
			while read crate; do \
				echo "  - $$crate"; \
			done < "$$tmpfile"; \
			rm -f "$$tmpfile"; \
			exit 1; \
		else \
			echo ""; \
			echo "✅ All crates ok!"; \
			rm -f "$$tmpfile"; \
		fi \
	)

module-build:
	@( \
		tmpfile=$$(mktemp); \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/Cargo.toml" ]; then \
				printf "Building $$p... "; \
				if (cd "$$p" && cargo build >/dev/null 2>&1); then \
					echo "✓"; \
				else \
					echo "✗"; \
					echo "$$p" >> "$$tmpfile"; \
				fi; \
			fi; \
		done; \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/package.json" ]; then \
				printf "Building $$p... "; \
				if (cd "$$p" && bun i && bun run build) >/dev/null 2>&1; then \
					echo "✓"; \
				else \
					echo "✗"; \
					echo "$$p" >> "$$tmpfile"; \
				fi; \
			fi; \
		done; \
		if [ -s "$$tmpfile" ]; then \
			echo ""; \
			echo "❌ Failed modules:"; \
			while read crate; do \
				echo "  - $$crate"; \
			done < "$$tmpfile"; \
			rm -f "$$tmpfile"; \
			exit 1; \
		else \
			echo ""; \
			echo "✅ All modules built!"; \
			rm -f "$$tmpfile"; \
		fi \
	)

list :
	@( \
		tmpfile=$$(mktemp); \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/Cargo.toml" ]; then \
				echo "$$p" >> "$$tmpfile"; \
			fi; \
		done; \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/package.json" ]; then \
				echo "$$p" >> "$$tmpfile"; \
			fi; \
		done; \
		find . -maxdepth 1 -type d ! -path . | while read p; do \
			if [ -f "$$p/*.css" ]; then \
				echo "$$p" >> "$$tmpfile"; \
			fi; \
		done; \
		if [ -s "$$tmpfile" ]; then \
			echo ""; \
			echo "Modules: "; \
			while read module; do \
				echo "  - $$module"; \
			done < "$$tmpfile"; \
			rm -f "$$tmpfile"; \
		else \
			echo ""; \
			echo "No modules found"; \
			rm -f "$$tmpfile"; \
		fi \
	)
