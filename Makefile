# **************************************************************************** #make
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: rparodi <rparodi@student.42.fr>            +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2023/11/12 11:05:05 by rparodi           #+#    #+#              #
#    Updated: 2025/09/05 17:32:55 by maiboyer         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

# Objdir
BUILD_DIR		= $(shell realpath ./build)
SRC_DIR			=	./src
INCLUDE_DIR		=	./include

AS		= nasm
NAME	= libasm.a

LIST_SUBJECT = https://cdn.intra.42.fr/pdf/pdf/175581/en.subject.pdf
ATOI_SUBJECT = https://cdn.intra.42.fr/pdf/pdf/171993/en.subject.pdf
SUBJECT_URL  = https://cdn.intra.42.fr/pdf/pdf/160063/en.subject.pdf

-include 			./Filelist.mk

OBJ				=	$(addsuffix .o,$(addprefix $(BUILD_DIR)/,$(SRC_FILES)))
DEPS			=	$(addsuffix .d,$(addprefix $(BUILD_DIR)/,$(SRC_FILES)))

-include			$(DEPS)

.PHONY: all re clean fclean test

# Colors
GREEN = \033[32m
CYAN = \033[36m
GREY = \033[0;90m
RED = \033[0;31m
GOLD = \033[38;5;220m
END = \033[0m
BOLD = \033[1m
ITALIC = \033[3m
UNDERLINE = \033[4m

TESTS =

all: $(NAME);

libasm_bonus.a: $(NAME)
	cp $^ $@

$(NAME): $(BUILD_DIR)/$(NAME)
	@cp $(BUILD_DIR)/$(NAME) $(NAME)

$(BUILD_DIR)/$(NAME): $(OBJ)
	@/usr/bin/env echo -e "$(GREY) AR $(GOLD)$(NAME)\033[0m"
	@ar rcs $(BUILD_DIR)/$(NAME) $(OBJ)

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.s 
	@mkdir -p $(shell dirname $@)
	@/usr/bin/env echo -e "$(GREY) NASM $(GREEN)$<\033[0m"
	@nasm -f elf64 -g -w+all -I$(SRC_DIR) -MF "$(@:%.o=%.d)" -o "$@" "$<"

test: $(NAME)
	cargo test --manifest-path ./libasm_test/Cargo.toml $(TESTS)

subject: .subject.txt
	@bat --plain ./.subject.txt
subject_list: .subject_list.txt
	@bat --plain ./.subject_list.txt
subject_atoi: .subject_atoi.txt
	@bat --plain ./.subject_atoi.txt

.subject.txt:
	@curl $(SUBJECT_URL) | pdftotext -layout -nopgbrk -q - .subject.txt
.subject_list.txt:
	@curl $(LIST_SUBJECT) | pdftotext -layout -nopgbrk -q - .subject_list.txt
.subject_atoi.txt:
	@curl $(ATOI_SUBJECT) | pdftotext -layout -nopgbrk -q - .subject_atoi.txt

clean:
	@rm -rf $(BUILD_DIR)

fclean:
	@$(MAKE) --no-print-directory clean
	@rm -rf $(NAME)
	@rm -rf libasm_bonus.a

re: 
	@$(MAKE) --no-print-directory fclean
	@$(MAKE) --no-print-directory all


filelist:
	@rm -f Filelist.mk
	@printf '%-78s\\\n' "SRC_FILES =" > Filelist.mk
	@tree $(SRC_DIR) -ifF \
		| rg -v '\.mac\.s$$' \
		| rg '$(SRC_DIR)/(.*)\.s$$' --replace '$$1' \
		| sed -re 's/^(.*)_([0-9]+)$$/\1|\2/g' \
		| sort -t'|' --key=1,1 --key=2,2n \
		| sed -e's/|/_/' \
		| xargs printf '%-78s\\\n' >> Filelist.mk
	@echo "" >> Filelist.mk


fakelib: fclean
	mkdir -p $(BUILD_DIR)
	clang -fPIC -Wall -Wextra -Wpedantic -g3 -c fakelib/mandatory.c -o $(BUILD_DIR)/fake_mandatory.o
	#clang -fPIC -Wall -Wextra -Wpedantic -g3  -c fakelib/bonus.c -o $(BUILD_DIR)/fake_bonus.o
	ar rcs $(NAME) $(BUILD_DIR)/fake_mandatory.o
	ar rcs libasm_test.a $(BUILD_DIR)/fake_mandatory.o
