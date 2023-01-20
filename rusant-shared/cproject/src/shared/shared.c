#include <shared.h>
#include <stdlib.h>

struct key_spec *get_key() {
	struct key_spec *ks = (struct key_spec*) malloc(sizeof(struct key_spec*));
	for (int i = 0; i < 16; i++) {
		ks->key[i] = i + 32;
	}

	ks->type = (const char*)"dummy\0";
	return ks;
}

typedef void (*AddCallback)(int result, void *user_data);

void better_add_two_numbers(int a, int b, AddCallback cb, void *user_data)
{
    int result = a + b;
    cb(result, user_data);
}