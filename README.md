# b64
A tiny util for dealing with simple b64's on the command line. Quite a bit quicker to type than the alternative workflow I'm using most of the time which needs to be piped to or given a file.

### Example
```
b64 -e hello
aGVsbG8=

b64 -d aGVsbG8=
hello
```
 

### Instead of
```
pbpaste | base64 -d
```

or

```
echo -n "<someb64encodedstring>" | base64 -d
```

