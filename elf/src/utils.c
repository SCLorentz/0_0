#include "utils.h"
#include <stdlib.h>

void load_sections(FILE *file, Elf64_Ehdr *header)
{
    // Ir até o início da section header table
    fseek(file, header->e_shoff, SEEK_SET);

    int sections_amount = header->e_shnum;
    int sections_size = header->e_shentsize;

    // Alocar e ler a tabela inteira
    Elf64_Shdr *secoes = malloc(sections_size * sections_amount);
    fread(secoes, sections_size, sections_amount, file);

    // Ler a seção de nomes
    Elf64_Shdr secao_strtab = secoes[header->e_shstrndx];
    char *nomes = malloc(secao_strtab.sh_size);
    fseek(file, secao_strtab.sh_offset, SEEK_SET);
    fread(nomes, 1, secao_strtab.sh_size, file);

    for (int i = 0; i < sections_amount; i++) {
        const char *nome = nomes + secoes[i].sh_name;
        printf("  [%2d] %-20s | Addr: 0x%lx | Tamanho: %ld bytes\n", i, nome, secoes[i].sh_addr, secoes[i].sh_size);
    }

    free(nomes);
    free(secoes);
}

const char* type(uint16_t type)
{
    switch (type) {
        case ET_REL: return "Relocável";
        case ET_EXEC: return "Executável";
        case ET_DYN: return "Compartilhado";
        case ET_CORE: return "Core dump";
        default: return "Desconhecido";
    }
}

const char* arch(uint16_t maquina)
{
    switch (maquina) {
        case EM_386: return "Intel 80386 (x86)";
        case EM_X86_64: return "AMD x86-64";
        case EM_ARM: return "ARM";
        case EM_AARCH64: return "AArch64 (ARM 64 bits)";
        case EM_RISCV: return "RISC-V";
        default: return "Desconhecida";
    }
}