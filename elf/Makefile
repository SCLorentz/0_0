BIN = elf_reader
SRCS = src/main.c src/utils.c
OBJS = $(SRCS:.c=.o)
CC = gcc
CFLAGS = -Wall -Wextra -g

# Regra padrão
all: $(BIN)

# Compiles the program
$(BIN): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^
	rm -f $(OBJS)

# Limpar arquivos compilados
clean:
	rm -f $(BIN) *.o