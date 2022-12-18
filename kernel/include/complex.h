typedef struct
{
    int a;
    int b;
    int c;
    int d;
} complex_return;

inline complex_return returns_complex(void)
{
    complex_return ret;

    ret.a = 1;
    ret.b = 2;
    ret.c = 3;
    ret.d = 4;

    return ret;
}