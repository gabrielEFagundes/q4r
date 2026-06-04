/*
    I based myself on the geeksforgeeks implementation of a hashmap in C.
    My changes are purely based on performance and usage.

    https://www.geeksforgeeks.org/dsa/implementation-of-hash-table-in-c-using-separate-chaining/
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <hash.h>

void setNode(struct node* node, char* key, char* value){
    node->key = key;
    node->value = value;
    node->next = NULL;
    return;
}

void initializeMap(struct hashMap* map){
    map->capacity = 100; // just to make sure
    map->numOfElem = 0;

    map->arr = (struct node**) malloc(sizeof(struct node*) * map->capacity);
}

int hash(struct hashMap* map, char* key){
    int bIndex, sum = 0;

    for(int i = 0; i < strlen(key); i++){
        sum = (sum * 31 + key[i]) % map->capacity; // horner's method is way more efficient and removes the need of the factor variable.
    }

    bIndex = sum;
    return bIndex;
}

void map(struct hashMap* map, KeyValue keyvalue[]){
    int bIndex = hash(map, keyvalue->key);
    struct node* newNode = (struct node*) malloc(sizeof(struct node));

    for(int i = 0; i < (sizeof(keyvalue) / sizeof(keyvalue[0])); i++){
        setNode(newNode, keyvalue->key, keyvalue->value);
    }

    if(map->arr[bIndex] == NULL){
        map->arr[bIndex] = newNode;

    }else{
        newNode->next = map->arr[bIndex];
        map->arr[bIndex] = newNode;
    }
    return;
}

char* search(struct hashMap* map, char* key){
    int bIndex = hash(map, key);

    struct node* bHead = map->arr[bIndex];
    while(bHead != NULL){
        if(bHead->key == key){
            return bHead->value;
        }
        bHead = bHead->next;
    }

    return NULL;
}