#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define NUM_POSTS 40
#define NUM_WOODS 2
uint16_t posts[] = {
    60,  // 1
    65,  // 2
    72,  // 3
    78,  // 4
    87,  // 5
    98,  // 6
    108, // 7
    119, // 8
    127, // 9
    130, // 10
    135, // 11
    142, // 12
    151, // 13
    157, // 14
    162, // 15
    169, // 16
    174, // 17
    104, // 18
    110, // 19
    119, // 20
    127, // 21
    137, // 22
    145, // 23
    146, // 24
    151, // 25
    94,  // 26
    102, // 27
    110, // 28
    119, // 29
    126, // 30
    134, // 31
    140, // 32
    150, // 33
    155, // 34
    111, // 35
    118, // 36
    126, // 37
    132, // 38
    137, // 39
    137, // 40
};
uint16_t woods[] = {243, 304};
typedef struct ll ll;

struct ll {
    uint16_t data;
    ll *next;
};

uint16_t ll_index(ll *list, int index) {
	for (int i = 0; i < index; i++)
		list = list->next;
    return list->data;
}

uint16_t ll_index(ll *list, int index) {
	for (int i = 0; i < index-1; i++)
		list = list->next;
	bbbbbbbbbbbbbbbbb
    return list->data;
}


void ll_print(ll *list) {
    printf("[");
    for (; list != NULL; list = list->next) {
		printf("%d", list->data);
		if (list->next != NULL)
		    printf(", ");
	}
    printf("]\n");
}


ll *array_to_ll(uint16_t array[], int len) {
	ll *mem = calloc(sizeof(ll), len);
	for (int i = 0; i < len; i++) {
	    mem[i] = (ll){
				.data=array[i],
				.next= (i != len-1 ? (ll*)(mem+(i+1)) : NULL),
		};
    }
	return mem;
}

void enumerate_combos(ll **combos, uint64_t bitfield, uint16_t leftover, ll *posts, uint16_t wood) {
	for (int i = 0; i < NUM_POSTS; i++) {
		uint16_t len = ll_index(posts, i);
		if (len > leftover)
				continue;
		uint16_t new_leftover = leftover-len;
		uint64_t new_bitfield = bitfield | (1 << i);
		ll_remove(posts, i);
		ll_add(posts, i-1, len);
	}
}

int main() {
    ll *combos = NULL;
	
    ll *new_posts = array_to_ll(posts, NUM_POSTS);
	ll_print(new_posts);
    for (int i = 0; i < NUM_WOODS; i++) {
        enumerate_combos(&combos, 0, woods[i], new_posts, woods[i]);
    }

}

