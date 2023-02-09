#define _CAT(a, b) a##b
#define CAT(a, b) _CAT(a, b)

#define _STRINGIFY(s) #s
#define STRINGIFY(s) _STRINGIFY(s)
