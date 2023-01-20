#ifndef SHARED_H
#define SHARED_H
#endif

struct key_spec {
	char key[16];
	const char *type;
};

struct key_spec* get_key();
