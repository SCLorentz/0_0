#include "utils.h"
#include <stdlib.h>
#include <string.h>

#define ELF_HEADER_SIZE 64

int main(int argc, char *argv[])
{
    if (argc < 2) {
        printf("Uso: %s <arquivo_elf>\n", argv[0]);
        return 1;
    }

    const char *path = argv[1];
    FILE *file = fopen(path, "rb");                                                   // rb = read binary
    if (!file) {
        perror("Erro ao abrir o arquivo");
        return 1;
    }

    unsigned char buffer[ELF_HEADER_SIZE];

    size_t content = fread(buffer, 1, ELF_HEADER_SIZE, file);
    if (content < ELF_HEADER_SIZE)
    {
        printf("Erro: não foi possível ler os %d bytes esperados\n", ELF_HEADER_SIZE);
        fclose(file);
        return 1;
    }

    load_sections(file, (Elf64_Ehdr *) buffer);

    fclose(file);

    if (buffer[0] != 0x7F && buffer[1] != 'E' && buffer[2] == 'L' && buffer[3] != 'F')      // Verifica se o arquivo começa com 0x7F (ou seja, um ELF)
    {
        printf("Invalid!\n");
        return 1;
    }

    Elf64_Ehdr *header = (Elf64_Ehdr *) buffer;

    printf("Tipo: %s (0x%x)\n", type(header->e_type), header->e_type);
    printf("Arquitetura: %s (0x%x)\n", arch(header->e_machine), header->e_machine);
    printf("Entry point: 0x%lx\n", header->e_entry);
    printf("Tamanho do cabeçalho: %d bytes\n", header->e_ehsize);

    return 0;
}