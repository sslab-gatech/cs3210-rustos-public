QUESTIONS_DIRS := $(shell find . -type d -name "questions")

.PHONY: all check

all:
	@echo "usage: make check"

check:
	@okay=true; \
	for qdir in $(QUESTIONS_DIRS); do \
	    for file in "$${qdir}/"*; do \
		    if ! [ -s "$${file}" ]; then \
			  okay=false; \
			  echo "Question file '$${file}' is empty."; \
		    fi \
	    done \
	done; \
	if ! $$okay; then \
		echo "Questions remain unanswered. Aborting."; \
		exit 1; \
	else \
		echo "All questions appear to be answered."; \
	fi