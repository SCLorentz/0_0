#ifndef UTILS_H
#define UTILS_H

#include <stdint.h>
#include <stdio.h>
#include <elf.h>

const char* type(uint16_t tipo);
const char* arch(uint16_t maquina);
void load_sections(FILE *arquivo, Elf64_Ehdr *cabecalho);

#endif