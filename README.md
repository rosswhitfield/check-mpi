# check-mpi

This program gives an easy way to check the cpu binding of a `mpirun`
or `srun` command.

```bash
$ checkmpi
Hello from rank 0, on host. (core affinity = 0-7)
```

```bash
$ taskset -c 1,2,3,5,6,8,9,10,15 checkmpi
Hello from rank 0, on pc0117540. (core affinity = 1-3,5,6,8-10,15)
```

```bash
$ mpirun -n 2 checkmpi
Hello from rank 0, on host. (core affinity = 0,4)
Hello from rank 1, on host. (core affinity = 1,5)
```

```bash
S srun -N 2 -n 8 -c 8 --cpu-bind=cores checkmpi
Hello from rank 0, on node1. (core affinity = 0,1,2,3,32,33,34,35)
Hello from rank 6, on node2. (core affinity = 4,5,6,7,36,37,38,39)
Hello from rank 2, on node1. (core affinity = 4,5,6,7,36,37,38,39)
Hello from rank 3, on node1. (core affinity = 20,21,22,23,52,53,54,55)
Hello from rank 1, on node1. (core affinity = 16,17,18,19,48,49,50,51)
Hello from rank 5, on node2. (core affinity = 16,17,18,19,48,49,50,51)
Hello from rank 7, on node2. (core affinity = 20,21,22,23,52,53,54,55)
Hello from rank 4, on node2. (core affinity = 0,1,2,3,32,33,34,35)
```
