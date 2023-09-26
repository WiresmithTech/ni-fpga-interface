

## FAQ

### Why use the C generator instead of the bitfile directly?

I still want to investigate the bitfile directly but using the C generator should mean:

1. This project is independent of the version of LabVIEW used (unless a breaking change is made).
2. We can use the C libraries functions for handling the more complex data types knowing it is compatible.
3. (related to 1) we need at least some of the generated files, otherwise we would have to cache a version in the repo.

I do think parsing the bitfile may end up being easier than the C code so I will see how the complex type support evolves.
If it doesn't benefit from the C code then this could be considered, or making it an option.