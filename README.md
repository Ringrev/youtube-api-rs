# Youtube Api for Rust and Wasm

This crate is for front_end  and supposed to be used with the custom `http client` of your choice as long as it uses the [http-types](https://docs.rs/http-types/2.11.0/http_types/). 

There are already existing stable [projects](https://github.com/Byron/google-apis-rs) for using it on the backend.

The Youtube Api requires `api_key` and a `refresh_token`.

The implementation of youtube Api is done manually with the help of the [documentation](https://developers.google.com/youtube/v3/docs) and used also to introduce Rust and Wasm to new programmers.




## Todo 

- Milestone version 0.1.x
    
- [ ] implement abstraction for generic client using `http-types`.
- [ ] implement `seed::fetch` client with feature gate.
- [ ] implement error handling.
- [ ] implement videos.
- [ ] implements comments.
- [ ] implement channels.
- [ ] implement examples.


