#include <stdint.h>
#include <stdio.h>

struct ll {
    uint16_t data;
    struct ll *next;
};

#define NUM_POSTS 40
#define NUM_WOODS 2
int main() {
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
    ll *combos = NULL;
    for (int i = 0; i < NUM_WOODS; i++) {
        enumerate_combos(&combos, 0, woods[i], posts, woods[i]);
}

void enumerate_combos() {
for (int i = 0; i < NUM_POSTS; i++) {
}
