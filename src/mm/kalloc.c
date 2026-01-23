

void freerange(void *vstart, void *vend);


extern char end[];


struct run {
    struct run *next;
};


struct {
    struct run *freelist;
} kmem;


void 
kfree(char *v) {
    
}