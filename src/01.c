#include <stdio.h>
#include <stdlib.h>
#include <string.h> // maybe dont need
#include <unistd.h>

typedef struct Item {
    int *key; 
    int *value;
    struct Item *next;
} Item;

typedef struct Map {
    Item **table;
    size_t size;
} Map;


/* Map initializer */
Map *init_map(size_t size) {
    Map *map = malloc(sizeof(Map));
    map->size=size;
    map->table = malloc(size * sizeof(Item *));
    for (size_t i = 0; i < size; i++) {
        map->table[i] = NULL;
    }
    return map;
}

/* Hash function djb2 inspired */
unsigned int hash(int key, size_t size) {
    return key % size;
}

void map_insert(Map *map, int *key, int *value) {
    unsigned int index = hash(key, map->size);
    Item *new_item = malloc(sizeof(Item));
    new_item->key = key;
    new_item->value = value;
    new_item->next = map->table[index];
    map->table[index] = new_item;
}

int *map_get(Map *map, int *key) {
    unsigned int index = hash(key, map->size);
    Item *current = map->table[index];
    while (current) {
        if (current->key==key) {
            return current->value;
        }
        current = current->next;
    }
    return NULL;
}

int comp(const void* a, const void* b) {
    // If a is smaller than b, the positive value will be returned
    return (*(int*)a - *(int*)b);
}

int getNumLines(FILE *f, char *fileName) {
    f = fopen(fileName, "r");
    int lines = 0;
    while (!feof(f)) {
        char ch = fgetc(f);
        if (ch == '\n') {
          lines++;
        }
    }
    fclose(f);
    return lines;
}

void split(int* l, int* r, int size, FILE* f) {
    char line[128];
    int index = 0;
    while(fgets(line, 100, f)) { // probably fine.. just reading number of chars in a line
        char *token = strtok(line, "   ");
        int k = 0;
        while (token != NULL) {
            if (k == 0) {
                l[index] = atoi(token);
            } else {
                r[index] = atoi(token);
            }
            k++;
            token = strtok(NULL, "  ");
        }
        index++;
    }
}

int main() {
    FILE *file;
    char input[] = "input/01";
    int lines = getNumLines(file, input);
    file = fopen(input, "r"); // put in function ?
    int *left_loc = (int*)malloc(lines * sizeof(int));
    int *right_loc = (int*)malloc(lines * sizeof(int));
    split(left_loc, right_loc, lines, file);
    qsort(left_loc, lines, sizeof(int), comp);
    qsort(right_loc, lines, sizeof(int), comp);
    int dist = 0;
    for (int i = 0; i < lines; i++) {
        dist += abs(left_loc[i] - right_loc[i]);
    }
    fclose(file);
    printf("Distance %d\n", dist);
    
    // part 2
    //  get all keys from left location ids
    Map *map = init_map(lines);
    for (int i = 0; i < lines; i++) {
        map_insert(map, left_loc[i], 0); // initialize map with 0 values
    }

    // update keys with number of occurences
    for(int i = 0; i < lines; i++) {
        int count = map_get(map, right_loc[i]);
        map_insert(map, right_loc[i], count + 1);
    }

    int result = 0;
    for(int i = 0; i < lines; i++) {
        int key = left_loc[i];
        int count = map_get(map, key);
        result += (key * count);
    }

    printf("Result: %d\n", result);

    return 0;
}
