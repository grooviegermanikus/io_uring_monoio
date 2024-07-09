
## Compiole
* works with rust version 1.75

## Run on Linux
* Kernel 5.6+
* configure `memlock` on linux [docs](https://github.com/bytedance/monoio/blob/HEAD/docs/en/memlock.md)

```
max locked memory           (kbytes, -l) 66001704
```

## Benchmark results

```
# on ams81
wrote 134217728 bytes in 171.407369ms - 746.7590264453567Mbps
```

```
# on macbook
wrote 134217728 bytes in 54.42275ms - 2351.96Mbps
```

