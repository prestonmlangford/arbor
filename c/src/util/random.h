
#ifndef RANDOM_H
#define RANDOM_H

void rand_seed(int seed);
void rand_seed_realtime(void);
int rand_range(int lower, int upper);
int rand_bound(int bound);
int rand_bernoulli(float p);

#endif // RANDOM_H
