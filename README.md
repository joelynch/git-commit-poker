# Git Commit Poker

Make running git commit slightly more interesting.  Arguments are all forwarded to `git commit`.

```bash
$ commit-poker -am 'A fun commit'
Committing ...
done!
Your commit hash is ... 3444930
> FULL HOUSE!! - 2 x 3, 3 x 4
    Points:  162320
> Flush - all numbers
    Points:  95892
Total points: 258212
```

## TODO

- [ ] Fix probabilities
- [ ] Gameify terminal output
- [ ] Keep track of high scores etc
