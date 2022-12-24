# Contribution guidelines


## Fork/clone/pull

The typical workflow for contributing to `rmt.rs` is:

1. Fork the `main` branch from the [GitHub repository](https://github.com/AmineZouitine/rmt.rs).
2. Clone your fork locally.
3. Commit changes.
4. Push the changes to your fork.
5. Send a pull request from your fork back to the original `main` branch.

## Run test
We are testing on the same database so we should run a test on a single thread.
```sh
cargo test -- --test-threads=1
```
rmt is not yet stable, to avoid any problem, I advise you to test using the Dockerfile

```sh
docker build -t "Image_name" .
docker run -d Image_name 
>> id
docker exec -it id sh
```

### Contact
email: amine.zouitinegm@gmail.com

### RoadMap

You can look at the issues to fix some bugs or add new features to ask by the community. 

You can propose any idea via the issues and make me a PR :D 


