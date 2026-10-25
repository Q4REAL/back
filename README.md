<p align="center" >
    <img src="back.png" style="width: 40%; height: auto;">
</p>

# Back is Basically Forth
Back is a stack based, single pass programming language. It's a little toy language I made to experiment with interpreters. It's not intended to be a "real" language.

```
: recursive 
    ** [ 1 ] = ?
        [ done! ] ->>
    ;
    ** [ 1 ] != ?
        [ still~going... ] ->>
        [ 1 ] -
        recursive
    ;
;
    
[ 5 ] recursive
```

see the examples folder for a guide on syntax.

This language intents to be feature complete, even if it's not really that usable. Here's the roadmap:

- [x] control flow, built-in operations, stack managment
- [x] verbose mode and basic error handling
- [ ] external dependencies
- [ ] more advanced type control (like checking for numbers)
- [ ] actually helpful error messages
- [ ] advanced error handling
- [ ] a readable verbose mode
- [ ] FFI?