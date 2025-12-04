Time to complete: ~2.5 hours
Speed-optimized runtime: 100ms (`perf stat [PROG]`)
Speed-optimized max heap: 111kB (`valgrind --trace-children=yes --tool=massif [PROG] && ms_print massif.out.* | head -40`)
Size-optimized executable size: 313kB
