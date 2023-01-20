#ifndef SHARED_H
#define SHARED_H
#endif

struct key_spec {
	char key[16];
	const char *type;
};

struct key_spec* get_key();

typedef void (*AddCallback)(int result, void *user_data);

void better_add_two_numbers(int a, int b, AddCallback cb, void *user_data);