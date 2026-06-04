typedef struct {
    char* key;
    char* value;
} KeyValue;

struct node{
    char* key;
    char* value;
    struct node* next;
};

struct hashMap{
    int numOfElem, capacity;
    struct node** arr;
};

/// @brief Sets the node
/// @param node The node
/// @param key Node's key
/// @param value Node's value
void setNode(struct node* node, char* key, char* value);

/// @brief Initializes the hashmap (initial capacity of 100)
/// @param map The hashmap
void initializeHash(struct hashMap* map);

/// @brief Hashes the defined hashmap with the key ascii symbol
/// @param map The hashmap
/// @param key The key used to hash
/// @return The bucket index (int)
int hash(struct hashMap* map, char* key);

/// @brief Creates the hashmap with an array of key values (sorry, I didn't find a way to make it another way)
/// @param map The hashmap
/// @param keyvalue[] The KeyValue struct ({key, value})
void map(struct hashMap* map, KeyValue keyvalue[]);

/// @brief Searches the key on the specified hashmap
/// @param map The hashmap
/// @param key The key to be searched on the hashmap
/// @return The value from the key or NULL if no element is found.
char* search(struct hashMap* map, char* key);