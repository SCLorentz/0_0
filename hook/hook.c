// ! this code is 100% AI generated for educational purposes

#include <stdio.h>
#include <string.h>
#include <stdlib.h>  // Inclua esta linha para resolver os avisos

int main() {
    FILE *file;
    const char *original = "Hello, World!";
    const char *replacement = "Hooked!";
    size_t original_len = strlen(original);
    size_t replacement_len = strlen(replacement);

    // Abrir o arquivo binário no modo leitura e escrita
    file = fopen("hello", "r+b");
    if (file == NULL) {
        perror("Erro ao abrir o arquivo");
        return 1;
    }

    // Ler o arquivo para o buffer
    fseek(file, 0, SEEK_END);
    long file_size = ftell(file);
    fseek(file, 0, SEEK_SET);

    char *buffer = (char *)malloc(file_size);
    if (buffer == NULL) {
        perror("Erro ao alocar memória");
        fclose(file);
        return 1;
    }

    fread(buffer, 1, file_size, file);

    // Procurar e substituir a string no buffer
    for (long i = 0; i < file_size - original_len; i++) {
        if (memcmp(&buffer[i], original, original_len) == 0) {
            // Verificar se o tamanho da string substituta cabe no lugar
            if (replacement_len <= original_len) {
                // Substituir a string
                memcpy(&buffer[i], replacement, replacement_len);
                // Se a nova string for menor, preencher o restante com zero
                if (replacement_len < original_len) {
                    memset(&buffer[i + replacement_len], 0, original_len - replacement_len);
                }
                break;
            } else {
                printf("A nova string é maior do que a original, operação não suportada.\n");
                free(buffer);
                fclose(file);
                return 1;
            }
        }
    }

    // Voltar ao início do arquivo e escrever o buffer modificado
    fseek(file, 0, SEEK_SET);
    fwrite(buffer, 1, file_size, file);

    // Liberar memória e fechar o arquivo
    free(buffer);
    fclose(file);

    printf("Substituição realizada com sucesso.\n");
    return 0;
}